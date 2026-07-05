<!-- markdownlint-disable MD013 -->
<!-- markdownlint-disable MD033 -->

# solana

- [Solana](https://solana.com/ru/docs/intro/installation)
- [Anchor](https://www.anchor-lang.com/docs/installation)
- [LiteSVM](https://www.litesvm.com/)
- [Faucet](https://faucet.solana.com/) - крипта для разработки
- [Explorer](https://explorer.solana.com/address/5Gq76UhKLVUoaag598pMASJB5RPzxhF7tKkQUJ1xuU26?cluster=devnet)
- [best practice](https://github.com/solana-foundation/awesome-solana-ai)
- [Solana Development Skill for Claude Code](https://github.com/solana-foundation/solana-dev-skill)
  Структура типового контракта делится на три основные части:

- Макрос #[program] — главный модуль, где прописывается вся бизнес-логика и функции (например, перевод токенов или создание пользователя).
- Структуры контекста #[derive(Accounts)] — списки всех аккаунтов, которые функция будет читать или изменять. Здесь же описываются правила валидации (кто подписал транзакцию, хватает ли прав).
- Аккаунты данных — структуры, определяющие, как именно данные будут сохраняться в блокчейне.

```sh
echo 'export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc

# проверить существование кошелька
solana address

# создать кошелёк
solana-keygen new -o /home/i/.config/solana/id.json

# запросить крипту для тестов
solana airdrop 2

# создать проект
npx create-solana-dapp@latest

# установить ИИ: claude code
curl -fsSL https://claude.ai/install.sh | bash
source ~/.bashrc
claude --version

# установить Solana Development Skill for Claude Code
npx skills add https://github.com/solana-foundation/solana-dev-skill

# использовать ИИ
claude

# создать проект
anchor init project_name

# компелировать проект
anchor build

# задеплоить в сеть
anchor program deploy --provider.cluster devnet

# установить виртуалку для тестов
cargo add --dev litesvm
```
