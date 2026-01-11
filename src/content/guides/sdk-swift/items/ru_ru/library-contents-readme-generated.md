The FastComments Swift SDK состоит из нескольких модулей:

- **Client Module** - Автогенерируемый клиент API для FastComments REST APIs
  - Полные определения типов для всех моделей API
  - Как аутентифицированные (`DefaultAPI`), так и публичные (`PublicAPI`) эндпоинты
  - Полная поддержка async/await
  - См. [client/README.md](https://github.com/FastComments/fastcomments-swift/blob/main/client/README.md) для подробной документации по API

- **SSO Module** - Утилиты Single Sign-On на стороне сервера
  - Безопасная генерация токенов для аутентификации пользователей
  - Поддержка как простого, так и защищённого режимов SSO
  - Подпись токенов на основе HMAC-SHA256 с использованием CryptoKit