The FastComments Rust SDK складається з кількох модулів:

- **Клієнтський модуль** - API-клієнт для FastComments REST API
  - Повні визначення типів для всіх моделей API
  - Три API-клієнта, що покривають усі методи FastComments:
    - `default_api` (**DefaultApi**) - методи, що автентифікуються ключем API, для серверного використання
    - `public_api` (**PublicApi**) - публічні методи без ключа API, безпечні для виклику з браузерів та мобільних застосунків
    - `moderation_api` (**ModerationApi**) - методи, які забезпечують панель модератора, включно з модерацією коментарів (list, count, search, logs, export), діями модерації (remove/restore, flag, set review/spam/approval status, votes, reopen/close thread), банами (ban from a comment, undo, pre-ban summaries, ban status/preferences, banned-user counts) та значками й довірою (award/remove badges, manual badges, get/set trust factor, user internal profile). Кожен метод модерації приймає параметр `sso`, тож виклик може бути виконаний від імені модератора, автентифікованого через SSO.
  - Повна підтримка async/await з tokio
  - Див. [client/README.md](https://github.com/FastComments/fastcomments-rust/blob/main/client/README.md) для детальної документації API

- **Модуль SSO** - серверні утиліти Single Sign-On
  - Безпечна генерація токенів для автентифікації користувачів
  - Підтримка як простого, так і захищеного режимів SSO
  - Підписування токенів на основі HMAC-SHA256

- **Основні типи** - Спільні визначення типів та утиліти
  - Моделі коментарів та структури метаданих
  - Налаштування користувачів та орендарів
  - Допоміжні функції для загальних операцій