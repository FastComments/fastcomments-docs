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

### Инсталација

```bash
sudo make install
```

### Садржај библиотеке

Ова библиотека садржи генерисани API клијент и SSO алате који олакшавају рад са API-јем.

- [Документација библиотеке API клијента](https://github.com/FastComments/fastcomments-cpp/blob/master/client/README.md)

### Јавни и заштићени API-ји

За API клијента постоје три класе, `DefaultApi`, `PublicApi`, и `ModerationApi`. The `DefaultApi` садржи методе које захтевају ваш API кључ, а `PublicApi` садржи
методе које се могу позивати директно из прегледача/мобилног уређаја/итд. без аутентификације. The `ModerationApi` садржи методе које покрећу контролну таблу модератора - listing,
counting, searching, exporting and pulling logs for comments, акције модерирања (remove/restore, flag, set review/spam/approval status, adjust votes, reopen/close threads),
забране (ban from a comment, undo bans, pre-ban summaries, ban status and preferences, banned-user counts), and badges & trust (award/remove badges, manual badges, get/set trust
factor, user internal profile). Every `ModerationApi` method accepts an `sso` parameter so the call is performed on behalf of an SSO-authenticated moderator.