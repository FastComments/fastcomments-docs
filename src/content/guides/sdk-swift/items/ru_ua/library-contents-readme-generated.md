FastComments Swift SDK состоит из нескольких модулей:

- **Клиентский модуль** - Автогенерированный клиент API для FastComments REST APIs
  - Полные определения типов для всех моделей API
  - Как аутентифицированные (`DefaultAPI`), так и публичные (`PublicAPI`) конечные точки
  - Полная поддержка async/await
  - См. [client/README.md](https://github.com/FastComments/fastcomments-swift/blob/main/client/README.md) для подробной документации по API

- **Модуль SSO** - Утилиты Single Sign-On на стороне сервера
  - Безопасная генерация токенов для аутентификации пользователей
  - Поддержка как простого, так и защищённого режимов SSO
  - Подпись токенов на основе HMAC-SHA256 с использованием CryptoKit