---
### PyPI

```bash
pip install fastcomments
```

### Содержимое библиотеки

Эта библиотека содержит два модуля: сгенерированный клиент API и основную Python-библиотеку, которая содержит вручную написанные утилиты, упрощающие работу с API, включая поддержку SSO.

- [Документация клиентской библиотеки API](https://github.com/FastComments/fastcomments-python/blob/main/client/README.md)
- [Документация основной библиотеки, включая примеры SSO](https://github.com/FastComments/fastcomments-python/blob/main/sso/README.md)

### Публичные vs Защищённые API

Для клиента API существуют два класса, `DefaultApi` и `PublicApi`. `DefaultApi` содержит методы, для которых требуется ваш API-ключ, а `PublicApi` содержит вызовы API, которые можно выполнять напрямую из браузера/мобильного устройства и т.д. без аутентификации.
---