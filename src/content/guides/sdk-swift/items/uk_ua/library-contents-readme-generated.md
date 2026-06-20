FastComments Swift SDK складається з кількох модулів:

- **Client Module** - клієнт API для FastComments REST APIs
  - Повні визначення типів для всіх моделей API
  - Автентифіковані (`DefaultAPI`), публічні (`PublicAPI`) та модераційні (`ModerationAPI`) методи
  - Повна підтримка async/await
  - Див. [client/README.md](https://github.com/FastComments/fastcomments-swift/blob/main/client/README.md) для детальної документації API

- **SSO Module** - серверні утиліти Single Sign-On
  - Безпечне генерування токенів для автентифікації користувачів
  - Підтримка як простого, так і захищеного режимів SSO
  - Підписування токенів на основі HMAC-SHA256 із використанням CryptoKit