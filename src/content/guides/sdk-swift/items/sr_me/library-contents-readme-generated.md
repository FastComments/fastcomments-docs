FastComments Swift SDK се састоји од неколико модула:

- **Клијент модул** - API клијент за FastComments REST API-је
  - Потпуне типске дефиниције за све API моделе
  - Аутентификоване (`DefaultAPI`), јавне (`PublicAPI`) и модерацијске (`ModerationAPI`) методе
  - Пуна подршка за async/await
  - Погледајте [client/README.md](https://github.com/FastComments/fastcomments-swift/blob/main/client/README.md) за детаљну документацију API-ја

- **SSO модул** - Серверске алатке за Single Sign-On
  - Сигурна генерација токена за аутентификацију корисника
  - Подршка и за једноставни и за сигурни режим SSO
  - Потписивање токена засновано на HMAC-SHA256 користећи CryptoKit