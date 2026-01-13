---
### PyPI

```bash
pip install fastcomments
```

### Садржај библиотеке

Ова библиотека садржи два модула: генерисани API клијент и основну Python библиотеку која садржи ручно написане услужне функције које олакшавају рад са API-јем, укључујући подршку за SSO.

- [Документација API клијента](https://github.com/FastComments/fastcomments-python/blob/main/client/README.md)
- [Документација основне библиотеке, укључујући примере SSO-а](https://github.com/FastComments/fastcomments-python/blob/main/sso/README.md)

### Јавни и заштићени API-ји

За API клијента постоје две класе, `DefaultApi` и `PublicApi`. `DefaultApi` садржи методе које захтевају ваш API кључ, а `PublicApi` садржи API позиве који се могу извршити директно из прегледача/мобилног уређаја/итд. без аутентификације.
---