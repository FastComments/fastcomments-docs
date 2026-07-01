### Using Nimble

```bash
nimble install fastcomments
```

### Building from Source

```bash
nimble build
```

### Library Contents

Эта библиотека содержит сгенерированный клиент API и утилиты SSO, упрощающие работу с API.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-nim/blob/master/client/README.md)

### Public vs Secured APIs

Для клиента API существуют три модуля API, `api_default`, `api_public` и `api_moderation`. `api_default` содержит методы, требующие ваш API‑ключ, а `api_public` содержит вызовы API
которые можно выполнять напрямую из браузера/мобильного устройства и т.д. без аутентификации. Модуль `api_moderation` содержит методы для панели модератора.

Модуль `api_moderation` предоставляет обширный набор быстрых и живых API модерации. Каждый метод `api_moderation` принимает параметр `sso` и может аутентифицироваться через SSO или сессионную cookie FastComments.com.