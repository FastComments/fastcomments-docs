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

Эта библиотека содержит сгенерированный API-клиент и утилиты SSO, чтобы упростить работу с API.

- [Документация библиотеки клиента API](https://github.com/FastComments/fastcomments-ruby/blob/master/client/README.md)

### Публичные и защищённые API

Для API-клиента есть два класса, `DefaultApi` и `PublicApi`. Класс `DefaultApi` содержит методы, требующие ваш API-ключ, а `PublicApi` содержит вызовы API, которые можно выполнять непосредственно из браузера/мобильного устройства и т.д. без аутентификации.