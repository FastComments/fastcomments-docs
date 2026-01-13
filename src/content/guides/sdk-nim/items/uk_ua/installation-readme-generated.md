### Використання Nimble

```bash
nimble install fastcomments
```

### Збірка з вихідного коду

```bash
nimble build
```

### Вміст бібліотеки

Ця бібліотека містить згенерований клієнт API та утиліти SSO, щоб полегшити роботу з API.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-nim/blob/master/client/README.md)

### Публічні й захищені API

Для клієнта API існує два модулі API, `api_default` та `api_public`. Модуль `api_default` містить методи, які вимагають вашого API-ключа, а `api_public` містить виклики API
які можна виконувати безпосередньо з браузера/мобільного пристрою тощо без автентифікації.