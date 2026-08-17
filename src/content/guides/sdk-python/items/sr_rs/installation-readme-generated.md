### Инсталирање са GitHub-а

Инсталирајте директно са ознаке издања (препоручено, потпуно репродуцибилно):

```bash
pip install git+https://github.com/fastcomments/fastcomments-python.git@v3.1.0
```

Закачите ознаку уместо гране како би изградње биле детерминисане. Исти формат функционише у `requirements.txt`:

```
fastcomments @ git+https://github.com/fastcomments/fastcomments-python.git@v3.1.0
```

Свака ознака [GitHub Release](https://github.com/FastComments/fastcomments-python/releases) такође има прикачен готов wheel ако више волите да директно инсталирате бинарни артефакт.

### Садржај библиотеке

Ова библиотека садржи два модула: генерисани API клијент и основну Python библиотеку која садржи ручно написане алате за олакшавање рада са API-јем, укључујући SSO подршку.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-python/blob/main/client/README.md)
- [Core Library Docs, Including SSO Examples](https://github.com/FastComments/fastcomments-python/blob/main/sso/README.md)

### Јавни vs Заштићени API-ји

За API клијент постоје три класе, `DefaultApi`, `PublicApi` и `ModerationApi`. `DefaultApi` садржи методе који захтевају ваш API кључ, а `PublicApi` садржи методе које се могу позивати директно из прегледача/мобилног уређаја/итд. без аутентификације. `ModerationApi` пружа обиман сет живих и брзих API-ја за модерацију. Сваки `ModerationApi` метод прихвата параметар `sso` и може се аутентификоваћe путем SSO или FastComments.com сесијског колачића.