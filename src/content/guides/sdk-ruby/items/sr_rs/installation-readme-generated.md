Add this line to your application's Gemfile:

```ruby
gem 'fastcomments'
```

And then execute:

```bash
bundle install
```

Or install it yourself as:

```bash
gem install fastcomments
```

### Library Contents

Ова библиотека садржи генерисаног API клијента и SSO алате који олакшавају рад са API-јем.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-ruby/blob/master/client/README.md)

### Public vs Secured APIs

За API клијента постоје три класе: `DefaultApi`, `PublicApi` и `ModerationApi`. `DefaultApi` садржи методе који захтевају ваш API кључ, а `PublicApi` садржи API позиве који се могу извршити директно из претраживача/мобилног уређаја итд. без аутентификације. `ModerationApi` садржи методе који покрећу контролни панел модератора.

`ModerationApi` пружа обиман скуп живих и брзих API-ја за модерацију. Сваки `ModerationApi` метод прихвата параметар `sso` и може се аутентификова​​ти путем SSO‑а или FastComments.com сесијског колачића.