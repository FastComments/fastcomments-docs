Добавете този ред към Gemfile на вашето приложение:

```ruby
gem 'fastcomments'
```

И след това изпълнете:

```bash
bundle install
```

Или инсталирайте сами чрез:

```bash
gem install fastcomments
```

### Съдържание на библиотеката

Тази библиотека съдържа генерирания API клиент и SSO помощни средства, които улесняват работата с API.

- [Документация на API клиентската библиотека](https://github.com/FastComments/fastcomments-ruby/blob/master/client/README.md)

### Публични срещу защитени API

За API клиента има три класа: `DefaultApi`, `PublicApi` и `ModerationApi`. `DefaultApi` съдържа методи, които изискват вашия API ключ, а `PublicApi` съдържа API повиквания, които могат да се извършват директно от браузър/мобилно устройство и т.н. без автентикация. `ModerationApi` съдържа методите, които захранват таблото за модериране.

`ModerationApi` предоставя обширен набор от API за живо и бързо модериране. Всеки метод на `ModerationApi` приема параметър `sso` и може да се автентикира чрез SSO или чрез сесийна бисквитка на FastComments.com.