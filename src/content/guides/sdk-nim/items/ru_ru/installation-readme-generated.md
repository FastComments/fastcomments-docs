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

- [Документация библиотеки клиента API](https://github.com/FastComments/fastcomments-nim/blob/master/client/README.md)

### Публичные и защищённые API

Для клиента API существуют два модуля API, `api_default` и `api_public`. `api_default` содержит методы, которые требуют ваш API-ключ, а `api_public` содержит вызовы API, которые можно выполнять прямо из браузера/мобильного устройства и т.д. без аутентификации.