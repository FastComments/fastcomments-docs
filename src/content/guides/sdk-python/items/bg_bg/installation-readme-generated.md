### Инсталиране от GitHub

Инсталирайте директно от етикет на версия (препоръчително, напълно възпроизведимо):

```bash
pip install git+https://github.com/fastcomments/fastcomments-python.git@v3.1.0
```

Закотвяйте етикета вместо клон, за да бъдат изграждането детерминистично. Същият формат работи в `requirements.txt`:

```
fastcomments @ git+https://github.com/fastcomments/fastcomments-python.git@v3.1.0
```

Всеки етикетиран [GitHub Release](https://github.com/fastcomments/fastcomments-python/releases) също има прикачен готов wheel, ако предпочитате да инсталирате бинарен артефакт директно.

### Съдържание на библиотеката

Тази библиотека съдържа два модула: генерирания API клиент и основната Python библиотека, която съдържа ръчно написани помощни функции за по‑лесна работа с API, включително поддръжка на SSO.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-python/blob/main/client/README.md)
- [Core Library Docs, Including SSO Examples](https://github.com/FastComments/fastcomments-python/blob/main/sso/README.md)

### Публични vs Защитени API

За API клиента има три класа – `DefaultApi`, `PublicApi` и `ModerationApi`. `DefaultApi` съдържа методи, които изискват вашия API ключ, а `PublicApi` съдържа методи, които могат да се извикат директно от браузър/мобилно устройство и т.н. без удостоверяване. `ModerationApi` предоставя обширен набор от живи и бързи API за модериране. Всеки метод на `ModerationApi` приема параметър `sso` и може да се удостоверява чрез SSO или с FastComments.com сесийна бисквитка.