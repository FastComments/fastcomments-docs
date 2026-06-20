FastComments Rust SDK се састоји од неколико модула:

- **Client Module** - API клијент за FastComments REST API-је
  - Комплетне дефиниције типова за све API моделе
  - Три API клијента који покривају све FastComments методе:
    - `default_api` (**DefaultApi**) - методе аутентификоване помоћу API кључа за коришћење на серверској страни
    - `public_api` (**PublicApi**) - јавне методе без API кључа које је безбедно позивати из прегледача и мобилних апликација
    - `moderation_api` (**ModerationApi**) - методе које подржавају контролну таблу модератора, укључући модерацију коментара (list, count, search, logs, export), акције модерације (remove/restore, flag, set review/spam/approval status, votes, reopen/close thread), забране (ban from a comment, undo, pre-ban summaries, ban status/preferences, banned-user counts), и значке & поверење (award/remove badges, manual badges, get/set trust factor, user internal profile). Свака Moderation метода прихвата параметар `sso` тако да позив може бити извршен у име модератора аутентификованог преко SSO.
  - Пуна async/await подршка уз tokio
  - Погледајте [client/README.md](https://github.com/FastComments/fastcomments-rust/blob/main/client/README.md) за детаљну API документацију

- **SSO Module** - серверске алатке за Single Sign-On
  - Сигурна генерација токена за аутентификацију корисника
  - Подршка за оба режима SSO: једноставни и сигурни
  - Потписивање токена засновано на HMAC-SHA256

- **Core Types** - Заједничке дефиниције типова и алатке
  - Модели коментара и структуре метаподатака
  - Конфигурације корисника и tenant-а
  - Помоћне функције за уобичајене операције