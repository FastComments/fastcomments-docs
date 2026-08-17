### Инсталирање зависности

```bash
sudo apt install libcpprest-dev libboost-all-dev
```

### Грађење из извора

```bash
mkdir build
cd build
cmake ..
make
```

### Инсталирање

```bash
sudo make install
```

### Садржај библиотеке

Ова библиотека садржи генерисани API клијент и SSO алате који олакшавају рад са API-јем.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-cpp/blob/master/client/README.md)

### Јавни vs заштићени API-ји

За API клијент постоје три класе, `DefaultApi`, `PublicApi` и `ModerationApi`. `DefaultApi` садржи методе који захтевају ваш API кључ, а `PublicApi` садржи методе које се могу позивати директно из прегледача/мобилног уређаја/итд без аутентификације. `ModerationApi` пружа обиман сет живих и брзих API-ја за модерацију. Сваки `ModerationApi` метод прихвата параметар `sso` и може се аутентификоваћ путем SSO или FastComments.com сесијског колачића.