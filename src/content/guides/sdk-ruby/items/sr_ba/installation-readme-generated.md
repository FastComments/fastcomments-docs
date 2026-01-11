---
Додајте ову линију у Gemfile ваше апликације:

```ruby
gem 'fastcomments'
```

А затим извршите:

```bash
bundle install
```

Или га инсталирајте сами као:

```bash
gem install fastcomments
```

### Садржај библиотеке

Ова библиотека садржи генерисан API клијент и SSO алатке које олакшавају рад са API-јем.

- [Документација библиотеке API клијента](https://github.com/FastComments/fastcomments-ruby/blob/master/client/README.md)

### Јавни и заштићени API-ји

За API клијента постоје двије класе, `DefaultApi` и `PublicApi`. `DefaultApi` садржи методе које захтијевају ваш API кључ, а `PublicApi` садржи позиве API-ја
који се могу направити директно из прегледача/мобилног уређаја/итд. без аутентификације.
---