Добавьте эту строку в Gemfile вашего приложения:

```ruby
gem 'fastcomments'
```

Затем выполните:

```bash
bundle install
```

Или установите самостоятельно:

```bash
gem install fastcomments
```

### Содержимое библиотеки

Эта библиотека содержит сгенерированный API-клиент и утилиты SSO для упрощения работы с API.

- [Документация клиентской библиотеки API](https://github.com/FastComments/fastcomments-ruby/blob/master/client/README.md)

### Публичные и защищённые API

Для API-клиента существует три класса: `DefaultApi`, `PublicApi` и `ModerationApi`. `DefaultApi` содержит методы, требующие ваш API-ключ, а `PublicApi` содержит вызовы API,
которые можно выполнять напрямую из браузера/мобильного устройства и т. д. без аутентификации. `ModerationApi` содержит методы, на которых работает панель модератора.

`ModerationApi` предоставляет обширный набор быстрых API для модерации в реальном времени. Каждый метод `ModerationApi` принимает параметр `sso` и может проходить аутентификацию через SSO или cookie сессии FastComments.com.