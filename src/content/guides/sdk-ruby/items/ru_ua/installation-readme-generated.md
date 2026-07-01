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

Эта библиотека содержит сгенерированный клиент API и утилиты SSO, упрощающие работу с API.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-ruby/blob/master/client/README.md)

### Public vs Secured APIs

Для клиента API существует три класса: `DefaultApi`, `PublicApi` и `ModerationApi`. `DefaultApi` содержит методы, требующие ваш API‑ключ, а `PublicApi` включает вызовы API, которые можно выполнять напрямую из браузера/мобильного устройства и т.д. без аутентификации. `ModerationApi` содержит методы, используемые в панели модератора.

`ModerationApi` предоставляет обширный набор быстрых и живых API модерации. Каждый метод `ModerationApi` принимает параметр `sso` и может аутентифицироваться через SSO или сессионный cookie FastComments.com.