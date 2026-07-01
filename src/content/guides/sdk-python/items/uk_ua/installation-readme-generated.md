### Встановити з GitHub

Встановіть безпосередньо з тегу випуску (рекомендовано, повністю відтворювально):

```bash
pip install git+https://github.com/fastcomments/fastcomments-python.git@v3.0.0
```

Закріпіть тег замість гілки, щоб збірки були детерміновані. Така ж форма працює у `requirements.txt`:

```
fastcomments @ git+https://github.com/fastcomments/fastcomments-python.git@v3.0.0
```

Кожен позначений [GitHub Release](https://github.com/fastcomments/fastcomments-python/releases) також має прикріплене зібране wheel, якщо ви віддаєте перевагу встановленню бінарного артефакту безпосередньо.

### Вміст бібліотеки

Ця бібліотека містить два модулі: згенерований API‑клієнт та основну Python‑бібліотеку, що містить написані вручну утиліти для спрощення роботи з API, включно з підтримкою SSO.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-python/blob/main/client/README.md)
- [Core Library Docs, Including SSO Examples](https://github.com/FastComments/fastcomments-python/blob/main/sso/README.md)

### Публічні та захищені API

Для API‑клієнта існує три класи, `DefaultApi`, `PublicApi` та `ModerationApi`. `DefaultApi` містить методи, які вимагають ваш API‑ключ, а `PublicApi` містить методи, які можна викликати безпосередньо з браузера/мобільного пристрою тощо без автентифікації. `ModerationApi` надає широку сукупність живих та швидких API модерації. Кожен метод `ModerationApi` приймає параметр `sso` і може автентифікуватись за допомогою SSO або cookie сесії FastComments.com.