Додайте цей рядок до Gemfile вашого додатка:

```ruby
gem 'fastcomments'
```

А потім виконайте:

```bash
bundle install
```

Або встановіть його самостійно так:

```bash
gem install fastcomments
```

### Зміст бібліотеки

Ця бібліотека містить згенерований клієнт API та утиліти SSO, щоб полегшити роботу з API.

- [Документація клієнтської бібліотеки API](https://github.com/FastComments/fastcomments-ruby/blob/master/client/README.md)

### Публічні та захищені API

Для клієнта API є два класи, `DefaultApi` та `PublicApi`. `DefaultApi` містить методи, які вимагають вашого ключа API, а `PublicApi` містить виклики API, які можна виконувати безпосередньо з браузера/мобільного пристрою тощо без автентифікації.