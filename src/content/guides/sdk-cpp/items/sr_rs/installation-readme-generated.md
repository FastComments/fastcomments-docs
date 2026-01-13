### Инсталирање зависности

```bash
sudo apt install libcpprest-dev libboost-all-dev
```

### Изградња из извора

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

Ова библиотека садржи генерисан API клијент и SSO алатке које олakшавају рад са API-јем.

- [Документација API клијент библиотеке](https://github.com/FastComments/fastcomments-cpp/blob/master/client/README.md)

### Јавни и заштићени API-ји

За API клијента постоје две класе, `DefaultAPI` и `PublicAPI`. `DefaultAPI` садржи методе које захтевају ваш API кључ, а `PublicAPI` садржи API позиве
који се могу извршити директно из прегледача/мобилног уређаја/итд. без аутентификације.