### Инсталиране от GitHub

Инсталирайте директно от етикет на версия (препоръчително, напълно възпроизведимо):

```bash
pip install git+https://github.com/fastcomments/fastcomments-python.git@v3.1.0
```

Закрепете етикета вместо клон, за да бъдат изграждането детерминистични. Същият формат работи в `requirements.txt`:

```
fastcomments @ git+https://github.com/fastcomments/fastcomments-python.git@v3.1.0
```

Всяко етикетирано [GitHub Release](https://github.com/fastcomments/fastcomments-python/releases) също има прикачено готово колело, ако предпочитате да инсталирате бинарен артефакт директно.

### Съдържание на библиотеката

Тази библиотека съдържа два модула: генерирания API клиент и основната Python библиотека, която съдържа ръчно написани помощни функции за улесняване на работата с API, включително поддръжка на SSO.

- [Документация за API клиентската библиотека](https://github.com/FastComments/fastcomments-python/blob/main/client/README.md)
- [Документация за основната библиотека, включително примери за SSO](https://github.com/FastComments/fastcomments-python/blob/main/sso/README.md)

### Публични vs Защитени API

За API клиента има три класа, `DefaultApi`, `PublicApi` и `ModerationApi`. `DefaultApi` съдържа методи, които изискват вашия API ключ, а `PublicApi` съдържа методи, които могат да се извикат директно от браузър/мобилно устройство/и т.н. без удостоверяване. `ModerationApi` предоставя обширен набор от живи и бързи API за модериране. Всеки метод на `ModerationApi` приема параметър `sso` и може да се удостоверява чрез SSO или с FastComments.com сесийна бисквитка.