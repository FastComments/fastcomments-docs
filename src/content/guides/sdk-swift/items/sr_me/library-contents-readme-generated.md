The FastComments Swift SDK се састоји из неколико модула:

- **Client Module** - Аутоматски генерисан API клијент за FastComments REST API-је
  - Комплетне типске дефиниције за све API моделе
  - И аутентификоване (`DefaultAPI`) и јавне (`PublicAPI`) крајње тачке
  - Пуна подршка за async/await
  - Погледајте [client/README.md](https://github.com/FastComments/fastcomments-swift/blob/main/client/README.md) за детаљну API документацију

- **SSO Module** - Серверски алати за Single Sign-On
  - Безбједно генерисање токена за аутентификацију корисника
  - Подршка за оба режима SSO: једноставан и сигуран
  - Потписивање токена засновано на HMAC-SHA256 користећи CryptoKit