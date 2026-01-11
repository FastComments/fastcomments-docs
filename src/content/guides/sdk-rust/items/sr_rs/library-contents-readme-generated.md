---
FastComments Rust SDK се састоји од неколико модула:

- **Client Module** - Аутоматски генерисан API клијент за FastComments REST API-је
  - Потпуне дефиниције типова за све API моделе
  - И аутентификоване (`DefaultApi`) и јавне (`PublicApi`) крајње тачке
  - Потпуна подршка за async/await уз tokio
  - Погледајте [client/README.md](https://github.com/FastComments/fastcomments-rust/blob/main/client/README.md) за детаљну API документацију

- **SSO Module** - Серверске алатке за Single Sign-On
  - Безбедна генерација токена за аутентификацију корисника
  - Подршка за и једноставан и сигуран SSO режим
  - Потписивање токена засновано на HMAC-SHA256

- **Core Types** - Заједничке дефиниције типова и алатке
  - Модели коментара и структуре метаподатака
  - Конфигурације корисника и tenant-а
  - Помоћне функције за уобичајене операције
---