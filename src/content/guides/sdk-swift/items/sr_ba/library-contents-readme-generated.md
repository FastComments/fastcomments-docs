---
FastComments Swift SDK се састоји од неколико модула:

- **Клијентски модул** - API клијент за FastComments REST API-је
  - Комплетне дефиниције типова за све API моделе
  - Аутентификоване (`DefaultAPI`), јавне (`PublicAPI`) и модерацијске (`ModerationAPI`) методе
  - Потпуна подршка за async/await
  - Погледајте [client/README.md](https://github.com/FastComments/fastcomments-swift/blob/main/client/README.md) за детаљну API документацију

- **SSO модул** - Алати за Single Sign-On на серверској страни
  - Сигурна генерација token-а за аутентификацију корисника
  - Подршка за оба SSO режима: једноставни и сигурни
  - Потписивање token-а засновано на HMAC-SHA256 уз коришћење CryptoKit
---