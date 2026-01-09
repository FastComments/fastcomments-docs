### Коришћење Nimble

```bash
nimble install fastcomments
```

### Изградња из извора

```bash
nimble build
```

### Садржај библиотеке

Ова библиотека садржи генерисани API клијент и SSO алатке које олакшавају рад са API-јем.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-nim/blob/master/client/README.md)

### Јавни и заштићени API-ји

За API клијента постоје два API модула, `api_default` и `api_public`. `api_default` садржи методе које захтевају ваш API кључ, а `api_public` садржи API позиве
који се могу извршити директно из прегледача/мобилног уређаја/итд. без аутентификације.