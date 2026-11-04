Додајте овај ред у Gemfile ваше апликације:

```ruby
gem 'fastcomments'
```

Затим извршите:

```bash
bundle install
```

Или инсталирајте сами помоћу:

```bash
gem install fastcomments
```

### Садржај библиотеке

Ова библиотека садржи генерисани API клијент и SSO алатке које олакшавају рад са API-јем.

- [Документација API клијент библиотеке](https://github.com/FastComments/fastcomments-ruby/blob/master/client/README.md)

### Јавни насупрот заштићеним API-јима

За API клијент постоје три класе: `DefaultApi`, `PublicApi` и `ModerationApi`. `DefaultApi` садржи методе које захтевају ваш API кључ, док `PublicApi` садржи API позиве
који се могу извршити директно из прегледача/мобилног уређаја/итд. без аутентификације. `ModerationApi` садржи методе које покрећу контролну таблу модератора.

`ModerationApi` пружа опсежан скуп API-ја за живу и брзу модерацију. Свака метода `ModerationApi` прихвата `sso` параметар и може се аутентификовати путем SSO-а или FastComments.com колачића сесије.