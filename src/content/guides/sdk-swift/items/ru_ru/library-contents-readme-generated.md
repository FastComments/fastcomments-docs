---
The FastComments Swift SDK состоит из нескольких модулей:

- **Client Module** - Клиент API для FastComments REST APIs
  - Полные определения типов для всех моделей API
  - Аутентифицированные (`DefaultAPI`), публичные (`PublicAPI`), и модерационные (`ModerationAPI`) методы
  - Полная поддержка async/await
  - See [client/README.md](https://github.com/FastComments/fastcomments-swift/blob/main/client/README.md) for detailed API documentation

- **SSO Module** - Утилиты серверной реализации Single Sign-On
  - Безопасная генерация токенов для аутентификации пользователей
  - Поддержка как простого, так и защищённого режимов SSO
  - Подпись токенов на основе HMAC-SHA256 с использованием CryptoKit
---