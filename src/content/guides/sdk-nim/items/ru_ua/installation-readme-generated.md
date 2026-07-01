### Использование Nimble

```bash
nimble install fastcomments
```

### Сборка из исходников

```bash
nimble build
```

### Содержимое библиотеки

Эта библиотека содержит сгенерированный клиент API и утилиты SSO, упрощающие работу с API.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-nim/blob/master/client/README.md)

### Публичные и защищённые API

Для клиента API существует три модуля API, `api_default`, `api_public` и `api_moderation`. `api_default` содержит методы, требующие ваш API‑ключ, а `api_public` содержит api calls  
that can be made directly from a browser/mobile device/etc without authentication. Модуль `api_moderation` содержит методы для панели модератора.

`api_moderation` предоставляет обширный набор живых и быстрых API модерации. Каждый метод `api_moderation` принимает параметр `sso` и может аутентифицироваться через SSO или cookie сеанса FastComments.com.