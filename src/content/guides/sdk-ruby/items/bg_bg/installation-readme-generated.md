Добавете този ред в Gemfile-а на вашето приложение:

```ruby
gem 'fastcomments'
```

И след това изпълнете:

```bash
bundle install
```

Или го инсталирайте сами като:

```bash
gem install fastcomments
```

### Съдържание на библиотеката

Тази библиотека съдържа генерирания API клиент и SSO помощни средства, за да улесни работата с API.

- [Документация на библиотеката на API клиента](https://github.com/FastComments/fastcomments-ruby/blob/master/client/README.md)

### Публични и защитени API

За API клиента има два класа, `DefaultApi` и `PublicApi`. `DefaultApi` съдържа методи, които изискват вашия API ключ, а `PublicApi` съдържа извиквания на API, които могат да се извършват директно от браузър/мобилно устройство/и т.н. без удостоверяване.
---