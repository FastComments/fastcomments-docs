---
The FastComments Swift SDK складається з кількох модулів:

- **Client Module** - Автоматично згенерований клієнт API для REST API FastComments
  - Повні визначення типів для всіх моделей API
  - Як автентифіковані (`DefaultAPI`), так і публічні (`PublicAPI`) кінцеві точки
  - Повна підтримка async/await
  - Див. [client/README.md](https://github.com/FastComments/fastcomments-swift/blob/main/client/README.md) для детальної документації API

- **SSO Module** - Утиліти серверної реалізації Single Sign-On
  - Безпечна генерація токенів для автентифікації користувачів
  - Підтримка як простих, так і захищених режимів SSO
  - Підписування токенів на основі HMAC-SHA256 із використанням CryptoKit
---