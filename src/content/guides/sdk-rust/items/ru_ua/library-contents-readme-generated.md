The FastComments Rust SDK consists of several modules:

- **Client Module** - клиент API для FastComments REST API
  - Полные определения типов для всех моделей API
  - Три API‑клиента, покрывающих все методы FastComments:
    - `default_api` (**DefaultApi**) - методы, аутентифицируемые API‑ключом, для серверного использования
    - `public_api` (**PublicApi**) - публичные методы без API‑ключа, безопасные для вызова из браузеров и мобильных приложений
    - `moderation_api` (**ModerationApi**) - обширный набор живых и быстрых API модерации. Каждый метод модерации принимает параметр `sso` и может аутентифицироваться через SSO или cookie‑сеанс FastComments.com.
  - Полная поддержка async/await с Tokio
  - См. [client/README.md](https://github.com/FastComments/fastcomments-rust/blob/main/client/README.md) для детальной документации API

- **SSO Module** - утилиты единого входа (Single Sign-On) на стороне сервера
  - Безопасная генерация токенов для аутентификации пользователей
  - Поддержка как простого, так и защищённого режимов SSO
  - Подпись токенов на основе HMAC‑SHA256

- **Core Types** - общие определения типов и утилиты
  - Модели комментариев и структуры метаданных
  - Конфигурации пользователей и арендаторов
  - Вспомогательные функции для общих операций