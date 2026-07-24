### Встановити з GitHub

Встановіть безпосередньо з тегу випуску (рекомендовано, повністю відтворювано):

```bash
pip install git+https://github.com/fastcomments/fastcomments-python.git@v3.1.0
```

Закріпіть тег замість гілки, щоб збірки були детермінованими. Така ж форма працює у `requirements.txt`:

```
fastcomments @ git+https://github.com/fastcomments/fastcomments-python.git@v3.1.0
```

Кожен тегований [GitHub Release](https://github.com/fastcomments/fastcomments-python/releases) також має прикріплене зібране wheel, якщо ви віддаєте перевагу встановленню бінарного артефакту безпосередньо.

### Вміст бібліотеки

Ця бібліотека містить два модулі: згенерований клієнт API та основну бібліотеку Python, яка містить написані вручну утиліти, що спрощують роботу з API, включаючи підтримку SSO.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-python/blob/main/client/README.md)
- [Core Library Docs, Including SSO Examples](https://github.com/FastComments/fastcomments-python/blob/main/sso/README.md)

### Публічні та захищені API

Для клієнта API існує три класи: `DefaultApi`, `PublicApi` та `ModerationApi`. `DefaultApi` містить методи, які вимагають ваш API‑ключ, а `PublicApi` містить методи, які можна викликати безпосередньо з браузера/мобільного пристрою тощо без автентифікації. `ModerationApi` надає широкий набір живих та швидких API модерації. Кожен метод `ModerationApi` приймає параметр `sso` і може автентифікуватися через SSO або cookie сесії FastComments.com.