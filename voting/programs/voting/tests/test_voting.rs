// Атрибут указывает компилятору Rust, что весь последующий модуль нужно компилировать только при запуске тестов (cargo test).
#[cfg(test)]
// Объявление модуля с именем tests, внутри которого будут находиться тестовые функции.
mod tests {
    // Импорт базовых типов фреймворка Anchor: system_program (Системная программа Solana), AccountMeta (структура для описания метаданных аккаунта в инструкции) и Pubkey (публичный ключ/адрес аккаунта).
    use anchor_lang::prelude::{system_program, AccountMeta, Pubkey};
    // Импорт виртуальной машины LiteSVM для локального тестирования смарт-контрактов.
    use litesvm::LiteSVM;
    // Импорт структуры для генерации пары ключей (публичный + приватный), которая представляет пользователя/кошелек.
    use solana_keypair::Keypair;
    // Импорт Instruction (минимальная единица выполнения в Solana) и Message (набор инструкций, из которых формируется транзакция).
    use solana_message::{Instruction, Message};
    // Импорт трейта Signer, который позволяет подписывать транзакции с помощью Keypair.
    use solana_signer::Signer;
    // Импорт структуры Transaction, которая объединяет сообщение (Message) и подписи для отправки в сеть.
    use solana_transaction::Transaction;
    // Импорт стандартного инструмента Rust для удобной работы с файловыми путями.
    use std::path::PathBuf;
    // создает глобальный путь, вычисляемый один раз при первом обращении
    use std::sync::LazyLock;
    // Импортирт сида из констант программы
    use voting::constants::POLL_SEED;

    // Глобальный путь к папке target/deploy, вычисляемый один раз на лету
    static DEPLOY_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        if path.to_string_lossy().contains("programs") {
            path.pop();
            path.pop();
        }
        path.push("target/deploy");
        path
    });

    /// Получение Pubkey программы с использованием глобального пути
    fn get_program_id() -> Pubkey {
        let keypair_path = DEPLOY_DIR.join("voting-keypair.json");

        let program_keypair =
            solana_keypair::read_keypair_file(&keypair_path).unwrap_or_else(|_| {
                panic!(
                    "Не удалось найти ключ программы по пути: {:?}",
                    keypair_path
                )
            });

        Pubkey::from(program_keypair.pubkey().to_bytes())
    }
    // Атрибут #[test] маркерует функцию test_voting_flow как исполняемый тест.
    #[test]
    fn test_voting_flow() {
        // Создание и инициализация нового пустого экземпляра виртуальной машины LiteSVM.
        let mut svm = LiteSVM::new();

        // Получаем Pubkey динамически
        let program_id_pubkey: Pubkey = get_program_id();

        // Использование глобального пути для загрузки .so файла
        let so_path = DEPLOY_DIR.join("voting.so");

        // Загрузка скомпилированного контракта voting.so в виртуальную машину по адресу program_id_pubkey. Если файл не найден, тест падает с понятной ошибкой.
        svm.add_program_from_file(program_id_pubkey, &so_path)
            .unwrap_or_else(|_| panic!("Не удалось найти файл программы по пути: {:?}", so_path));

        // Генерация нового случайного кошелька (плательщика), который будет подписывать транзакции и платить за создание аккаунтов (комиссию).
        let payer = Keypair::new();
        // Начисление бесплатного баланса (airdrop) на кошелек payer в размере 10 миллиардов Лампортов (что равняется 10 SOL), чтобы у него были средства для тестов.
        svm.airdrop(&payer.pubkey(), 10_000_000_000).unwrap();

        // Определение тестовых данных: ID опроса равен 1, имя первого кандидата — "Candidate 1".
        let poll_id: u64 = 1;
        let candidate_name = "Candidate 1".to_string();

        // Вычисление PDA-адреса для аккаунта голосования.
        let (poll_pda, _) =
            Pubkey::find_program_address(&[POLL_SEED, &poll_id.to_le_bytes()], &program_id_pubkey);

        // Вычисление PDA-адреса для аккаунта кандидата.
        let (candidate_pda, _) = Pubkey::find_program_address(
            &[&poll_id.to_le_bytes(), candidate_name.as_bytes()],
            &program_id_pubkey,
        );

        // Создание динамического массива байт poll_args (аргументы вызова функции).
        // Первым делом туда записывается ID голосования (8 байт).
        let mut poll_args = Vec::new();
        poll_args.extend_from_slice(&poll_id.to_le_bytes());

        // Добавление строки названия голосования.
        // По стандарту сериализации (Borsh), используемому в Anchor,
        // сначала записывается длина строки как u32 (4 байта), а затем сами байты строки.
        let name = "Best Framework".to_string();
        poll_args.extend_from_slice(&(name.len() as u32).to_le_bytes());
        poll_args.extend_from_slice(name.as_bytes());

        // Добавление строки описания голосования (длина u32 + тело строки).
        let desc = "Choose your favorite".to_string();
        poll_args.extend_from_slice(&(desc.len() as u32).to_le_bytes());
        poll_args.extend_from_slice(desc.as_bytes());

        // Добавление временных меток начала (0) и окончания (Unix timestamp)
        // голосования в формате u64 (по 8 байт каждая).
        poll_args.extend_from_slice(&0u64.to_le_bytes()); // start_time
        poll_args.extend_from_slice(&1893456000u64.to_le_bytes()); // end_time

        // Первые 8 байт — это дискриминатор метода Anchor
        // (хэш SHA256 от названия функции создания голосования в контракте).
        // Anchor использует его для маршрутизации вызова.
        // Следом за ним прикрепляются все подготовленные poll_args.
        let mut poll_data = vec![76, 170, 187, 179, 118, 179, 228, 33];
        poll_data.extend(poll_args);

        // Список аккаунтов, необходимых для выполнения инструкции:
        // кошелек плательщика (изменяемый, подписывает true),
        // создаваемый PDA опроса (изменяемый, не подписывает false),
        // и неизменяемая Системная программа Solana (нужна для выделения памяти под PDA).
        let poll_accounts = vec![
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new(poll_pda, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ];

        // Сборка полноценного объекта Instruction для создания опроса.
        let poll_ix = Instruction {
            program_id: program_id_pubkey,
            accounts: poll_accounts,
            data: poll_data,
        };

        // Формирование аргументов кандидата: ID голосования (u64), длина имени кандидата (u32) и само имя.
        let mut cand_args = Vec::new();
        cand_args.extend_from_slice(&poll_id.to_le_bytes());
        cand_args.extend_from_slice(&(candidate_name.len() as u32).to_le_bytes());
        cand_args.extend_from_slice(candidate_name.as_bytes());

        // Инициализация вектора данных уникальным 8-байтовым дискриминатором Anchor
        // для функции добавления кандидата и добавление аргументов.
        let mut cand_data = vec![7, 127, 28, 59, 98, 162, 246, 17];
        cand_data.extend(cand_args);

        // Список аккаунтов для кандидата:
        // плательщик,
        // аккаунт опроса (для привязки/проверки),
        // новый PDA кандидата и Системная программа.
        let cand_accounts = vec![
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new(poll_pda, false),
            AccountMeta::new(candidate_pda, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ];

        // Сборка объекта Instruction для добавления кандидата.
        let cand_ix = Instruction {
            program_id: program_id_pubkey,
            accounts: cand_accounts,
            data: cand_data,
        };

        // Формирование аргументов для самого голоса
        // (аналогично: ID голосования и имя выбранного кандидата).
        let mut vote_args = Vec::new();
        vote_args.extend_from_slice(&poll_id.to_le_bytes());
        vote_args.extend_from_slice(&(candidate_name.len() as u32).to_le_bytes());
        vote_args.extend_from_slice(candidate_name.as_bytes());

        // Подготовка вектора данных с 8-байтовым дискриминатором метода голосования Anchor.
        let mut vote_data = vec![227, 110, 155, 23, 136, 126, 172, 25];
        vote_data.extend(vote_args);

        // Список аккаунтов для инкремента счетчика голосов:
        // голосующий плательщик, опрос и кандидат.
        // Системная программа тут не нужна, так как новые аккаунты не создаются.
        let vote_accounts = vec![
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new(poll_pda, false),
            AccountMeta::new(candidate_pda, false),
        ];

        // Сборка объекта Instruction для учета голоса.
        let vote_ix = Instruction {
            program_id: program_id_pubkey,
            accounts: vote_accounts,
            data: vote_data,
        };

        // Объединение всех трех созданных инструкций (poll_ix, cand_ix, vote_ix)
        // в единое атомарное сообщение Message.
        // Это означает, что либо все три действия выполнятся успешно,
        // либо вся транзакция откатится.
        // Вторым параметром передается адрес главного плательщика за транзакцию.
        let msg = Message::new(&[poll_ix, cand_ix, vote_ix], Some(&payer.pubkey()));
        // Создание готовой транзакции Transaction.
        // На вход подается массив подписантов (в данном случае один &payer), само сообщение
        // и актуальный хэш последнего блока из симулятора svm (защита от повторных транзакций).
        let tx = Transaction::new(&[&payer], msg, svm.latest_blockhash());

        // Отправка транзакции на исполнение внутрь виртуальной машины LiteSVM.
        let tx_result = svm.send_transaction(tx);
        // Главная проверка теста (assert!).
        // Если транзакция завершилась с ошибкой
        // (например, не хватило денег, не совпали PDA или логика контракта вернула ошибку),
        // тест прервется и выведет описание ошибки из tx_result.err().
        // Если все ок — тест успешно пройден.
        assert!(
            tx_result.is_ok(),
            "Transaction failed: {:?}",
            tx_result.err()
        );
    }
}
