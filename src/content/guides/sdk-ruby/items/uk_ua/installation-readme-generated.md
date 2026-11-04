Додайте цей рядок до Gemfile вашого застосунку:

```ruby
gem 'fastcomments'
```

А потім виконайте:

```bash
bundle install
```

Або встановіть його самостійно:

```bash
gem install fastcomments
```

### Вміст бібліотеки

Ця бібліотека містить згенерований API-клієнт та утиліти SSO, щоб спростити роботу з API.

- [Документація бібліотеки API-клієнта](https://github.com/FastComments/fastcomments-ruby/blob/master/client/README.md)

### Публічні та захищені API

Для API-клієнта є три класи: `DefaultApi`, `PublicApi` та `ModerationApi`. `DefaultApi` містить методи, які вимагають ваш API-ключ, а `PublicApi` містить API-виклики, які можна виконувати безпосередньо з браузера/мобільного пристрою тощо без автентифікації. `ModerationApi` містить методи, на яких працює панель модератора.

`ModerationApi` надає розширений набір швидких API для модерації в реальному часі. Кожен метод `ModerationApi` приймає параметр `sso` і може автентифікуватися через SSO або cookie сесії FastComments.com.