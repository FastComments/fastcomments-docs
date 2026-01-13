FastComments Swift SDK се састоји из неколико модула:

- **Client Module** - Аутоматски генерисан API клијент за FastComments REST API-је
  - Потпуне дефиниције типова за све API моделе
  - И аутентификоване (`DefaultAPI`) и јавне (`PublicAPI`) крајње тачке
  - Пуна подршка за async/await
  - Погледајте [client/README.md](https://github.com/FastComments/fastcomments-swift/blob/main/client/README.md) за детаљну документацију API-ја

- **SSO Module** - Серверске Single Sign-On (SSO) алатке
  - Безбедно генерисanje токена за аутентификацију корисника
  - Подршка за оба режима SSO-а: једноставни и сигурни
  - Потписивање токена засновано на HMAC-SHA256 уз употребу CryptoKit