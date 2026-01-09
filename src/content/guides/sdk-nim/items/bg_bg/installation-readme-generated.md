### Използване на Nimble

```bash
nimble install fastcomments
```

### Сграждане от изходния код

```bash
nimble build
```

### Съдържание на библиотеката

Тази библиотека съдържа генерирания API клиент и SSO помощни инструменти, които улесняват работата с API-то.

- [Документация на библиотеката за API клиента](https://github.com/FastComments/fastcomments-nim/blob/master/client/README.md)

### Публични и защитени API

За API клиента има два API модула, `api_default` и `api_public`. `api_default` съдържа методи, които изискват вашия API ключ, а `api_public` съдържа api повиквания
които могат да бъдат направени директно от браузър/мобилно устройство/и т.н. без автентикация.