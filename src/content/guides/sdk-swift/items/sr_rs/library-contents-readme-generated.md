The FastComments Swift SDK се састоји од неколико модула:

- **Client модул** - API клијент за FastComments REST API-је
  - Комплетне дефиниције типова за све API моделе
  - Аутентификоване (`DefaultAPI`), јавне (`PublicAPI`) и методе за модерацију (`ModerationAPI`)
  - Пуна подршка за async/await
  - Погледајте [client/README.md](https://github.com/FastComments/fastcomments-swift/blob/main/client/README.md) за детаљну API документацију

- **SSO модул** - алатке за Single Sign-On на серверској страни
  - Сигурна генерација токена за аутентификацију корисника
  - Подршка за оба режима SSO: једноставан и сигуран
  - Потписивање токена засновано на HMAC-SHA256 уз коришћење CryptoKit