### Використання Nimble

```bash
nimble install fastcomments
```

### Збірка з джерела

```bash
nimble build
```

### Вміст бібліотеки

Ця бібліотека містить згенерований клієнт API та утиліти SSO, які спрощують роботу з API.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-nim/blob/master/client/README.md)

### Публічні та захищені API

Для клієнта API існує три модулі API: `api_default`, `api_public` та `api_moderation`. `api_default` містить методи, які вимагають ваш API‑ключ, а `api_public` містить виклики API,
які можна виконувати безпосередньо з браузера/мобільного пристрою тощо без автентифікації. Модуль `api_moderation` містить методи для панелі модератора.

Модуль `api_moderation` надає розширений набір живих та швидких API модерації. Кожен метод `api_moderation` приймає параметр `sso` і може автентифікуватися через SSO або за допомогою cookie сесії FastComments.com.