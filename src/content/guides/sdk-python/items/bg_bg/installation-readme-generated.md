### PyPI

```bash
pip install fastcomments
```

### Съдържание на библиотеката

Тази библиотека съдържа два модула: генерирания API клиент и основната Python библиотека, която съдържа ръчно написани помощни инструменти, които улесняват работата с API-то, включително поддръжка на SSO.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-python/blob/main/client/README.md)
- [Core Library Docs, Including SSO Examples](https://github.com/FastComments/fastcomments-python/blob/main/sso/README.md)

### Публични и защитени API-та

За клиентската библиотека има два класа, `DefaultApi` и `PublicApi`. `DefaultApi` съдържа методи, които изискват вашия API ключ, а `PublicApi` съдържа повиквания към API, които могат да се извършват директно от браузър/мобилно устройство/и т.н. без удостоверяване.