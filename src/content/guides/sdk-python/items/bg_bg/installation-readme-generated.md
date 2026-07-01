### Инсталиране от GitHub

Инсталирайте директно от етикет на версия (препоръчително, напълно възпроизводимо):

```bash
pip install git+https://github.com/fastcomments/fastcomments-python.git@v3.0.0
```

Фиксирайте етикета вместо клон, за да бъдат компилациите детерминирани. Същият формат работи в `requirements.txt`:

```
fastcomments @ git+https://github.com/fastcomments/fastcomments-python.git@v3.0.0
```

Всеки етикетиран [GitHub Release](https://github.com/fastcomments/fastcomments-python/releases) също има прикачен готов wheel, ако предпочитате да инсталирате бинарен артефакт директно.

### Съдържание на библиотеката

Тази библиотека съдържа два модула: генерирания клиент за API и основната Python библиотека, която съдържа ръчно написани помощни функции за улесняване на работата с API‑то, включително поддръжка на SSO.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-python/blob/main/client/README.md)
- [Core Library Docs, Including SSO Examples](https://github.com/FastComments/fastcomments-python/blob/main/sso/README.md)

### Публични срещу защитени API‑та

За клиентския API има три класа – `DefaultApi`, `PublicApi` и `ModerationApi`. `DefaultApi` съдържа методи, които изискват вашия API ключ, а `PublicApi` съдържа методи, които могат да се извикат директно от браузър/мобилно устройство и пр. без автентификация. `ModerationApi` предоставя обширен набор от живи и бързи модерационни API‑та. Всеки метод на `ModerationApi` приема параметър `sso` и може да се автентикира чрез SSO или FastComments.com сесийна бисквитка.