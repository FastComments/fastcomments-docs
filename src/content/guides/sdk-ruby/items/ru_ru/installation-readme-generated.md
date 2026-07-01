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

### Содержимое библиотеки

Эта библиотека содержит сгенерированный клиент API и утилиты SSO, упрощающие работу с API.

- [Документация библиотеки API клиента](https://github.com/FastComments/fastcomments-ruby/blob/master/client/README.md)

### Публичные и защищённые API

Для клиента API существуют три класса: `DefaultApi`, `PublicApi` и `ModerationApi`. `DefaultApi` содержит методы, требующие ваш API‑ключ, а `PublicApi` содержит вызовы API, которые можно выполнять напрямую из браузера, мобильного устройства и т.п. без аутентификации. `ModerationApi` содержит методы, обеспечивающие работу панели модератора.

`ModerationApi` предоставляет широкий набор быстрых и живых API модерации. Каждый метод `ModerationApi` принимает параметр `sso` и может аутентифицироваться через SSO или cookie‑сеанс FastComments.com.