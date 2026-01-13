FastComments Swift SDK се састоји од неколико модула:

- **Client Module** - Аутоматски генерисан API клијент за FastComments REST API-је
  - Комплетне дефиниције типова за све API моделе
  - И аутентификоване (`DefaultAPI`) и јавне (`PublicAPI`) крајње тачке
  - Пуна подршка за async/await
  - Погледајте [client/README.md](https://github.com/FastComments/fastcomments-swift/blob/main/client/README.md) за детаљну API документацију

- **SSO Module** - Алати за Single Sign-On на серверској страни
  - Сигурно генерисање токена за аутентификацију корисника
  - Подршка за и једноставне и сигурне SSO режиме
  - Потписивање токена засновано на HMAC-SHA256 уз употребу CryptoKit