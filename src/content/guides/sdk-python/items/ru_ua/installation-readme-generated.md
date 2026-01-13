---
### PyPI

```bash
pip install fastcomments
```

### Содержимое библиотеки

Эта библиотека содержит два модуля: сгенерированный клиент API и основную библиотеку Python, которая содержит ручные утилиты, упрощающие работу с API, включая поддержку SSO.

- [Документация клиентской библиотеки API](https://github.com/FastComments/fastcomments-python/blob/main/client/README.md)
- [Документация основной библиотеки, включая примеры SSO](https://github.com/FastComments/fastcomments-python/blob/main/sso/README.md)

### Публичные и защищённые API

Для клиента API имеются два класса, `DefaultApi` и `PublicApi`. `DefaultApi` содержит методы, требующие ваш API-ключ, а `PublicApi` содержит вызовы API, которые можно выполнять напрямую из браузера/мобильного устройства и т. п. без аутентификации.
---