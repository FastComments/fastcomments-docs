### Использование Nimble

```bash
nimble install fastcomments
```

### Сборка из исходников

```bash
nimble build
```

### Состав библиотеки

Эта библиотека содержит сгенерированный API-клиент и утилиты SSO, упрощающие работу с API.

- [Документация библиотеки API-клиента](https://github.com/FastComments/fastcomments-nim/blob/master/client/README.md)

### Публичные и защищённые API

В API-клиенте есть три API-модуля: `api_default`, `api_public` и `api_moderation`. Модуль `api_default` содержит методы, требующие ваш API-ключ, а `api_public` содержит вызовы API,
которые можно выполнять напрямую из браузера/мобильного устройства и т.д. без аутентификации. Модуль `api_moderation` содержит методы для панели модератора.

Модуль `api_moderation` предоставляет обширный набор быстрых API для модерации в реальном времени. Каждый метод `api_moderation` принимает параметр `sso` и может выполнять аутентификацию через SSO или cookie сессии FastComments.com.