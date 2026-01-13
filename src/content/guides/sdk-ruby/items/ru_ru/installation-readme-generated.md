Добавьте эту строку в Gemfile вашего приложения:

```ruby
gem 'fastcomments'
```

А затем выполните:

```bash
bundle install
```

Или установите его самостоятельно с помощью:

```bash
gem install fastcomments
```

### Содержимое библиотеки

Эта библиотека содержит сгенерированный клиент API и утилиты SSO, упрощающие работу с API.

- [Документация клиентской библиотеки API](https://github.com/FastComments/fastcomments-ruby/blob/master/client/README.md)

### Публичные и защищённые API

Для клиента API существуют два класса: `DefaultApi` и `PublicApi`. `DefaultApi` содержит методы, которые требуют вашего ключа API, а `PublicApi` содержит вызовы API, которые можно выполнять напрямую из браузера, мобильного устройства и т.д. без аутентификации.
---