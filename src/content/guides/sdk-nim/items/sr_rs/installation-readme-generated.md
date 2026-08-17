### Коришћење Nimble

```bash
nimble install fastcomments
```

### Грађење из извора

```bash
nimble build
```

### Садржај библиотеке

Ова библиотека садржи генерисани API клијент и SSO алате који олакшавају рад са API-јем.

- [Документација API клијент библиотеке](https://github.com/FastComments/fastcomments-nim/blob/master/client/README.md)

### Јавни vs заштићени API-ји

За API клијент постоје три API модула, `api_default`, `api_public` и `api_moderation`. `api_default` садржи методе које захтевају ваш API кључ, а `api_public` садржи API позиве који се могу извршити директно из прегледача/мобилног уређаја/итд без аутентификације. `api_moderation` модул садржи методе за контролни панел модератора.

`api_moderation` модул пружа обиман сет живих и брзих API-ја за модерацију. Свака `api_moderation` метода прихвата `sso` параметар и може се аутентификоваћити преко SSO или FastComments.com сесијског колачића.