The FastComments Swift SDK состоит из нескольких модулей:

- **Модуль клиента** - Клиент REST API FastComments
  - Полные определения типов для всех моделей API
  - Аутентифицированные (`DefaultAPI`), публичные (`PublicAPI`) и модерационные (`ModerationAPI`) методы
  - Полная поддержка async/await
  - См. [client/README.md](https://github.com/FastComments/fastcomments-swift/blob/main/client/README.md) для подробной документации API

- **Модуль SSO** - Серверные утилиты единого входа (Single Sign-On)
  - Безопасная генерация токенов для аутентификации пользователей
  - Поддержка как простого, так и защищённого режима SSO
  - Подписание токенов на основе HMAC-SHA256 с использованием CryptoKit