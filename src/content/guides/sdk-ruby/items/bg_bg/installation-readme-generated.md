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

Тази библиотека съдържа генерирания клиент за API и SSO инструменти, които улесняват работата с API.

- [Документация за клиентската библиотека на API](https://github.com/FastComments/fastcomments-ruby/blob/master/client/README.md)

### Public vs Secured APIs

За клиентския API има три класа – `DefaultApi`, `PublicApi` и `ModerationApi`. `DefaultApi` съдържа методи, които изискват вашия API ключ, а `PublicApi` съдържа API повиквания, които могат да се направят директно от браузър/мобилно устройство/и т.н. без удостоверяване. `ModerationApi` съдържа методите, които захранват таблото за модератори.

`ModerationApi` предоставя обширен набор от живи и бързи API за модериране. Всеки метод на `ModerationApi` приема параметър `sso` и може да се удостоверява чрез SSO или с бисквитка от сесия на FastComments.com.