### Установка с GitHub

Установите напрямую из тега релиза (рекомендовано, полностью воспроизводимо):

```bash
pip install git+https://github.com/fastcomments/fastcomments-python.git@v3.1.0
```

Привязывайте тег, а не ветку, чтобы сборки были детерминированными. Та же форма работает в `requirements.txt`:

```
fastcomments @ git+https://github.com/fastcomments/fastcomments-python.git@v3.1.0
```

Каждый помеченный [GitHub Release](https://github.com/fastcomments/fastcomments-python/releases) также имеет прикреплённый готовый wheel, если вы предпочитаете установить бинарный артефакт напрямую.

### Содержание библиотеки

Эта библиотека содержит два модуля: сгенерированный клиент API и основную библиотеку Python, которая включает написанные вручную утилиты для упрощения работы с API, включая поддержку SSO.

- [Документация клиентской библиотеки API](https://github.com/FastComments/fastcomments-python/blob/main/client/README.md)
- [Документация основной библиотеки, включая примеры SSO](https://github.com/FastComments/fastcomments-python/blob/main/sso/README.md)

### Публичные и защищённые API

Для клиентской библиотеки API существует три класса: `DefaultApi`, `PublicApi` и `ModerationApi`. `DefaultApi` содержит методы, требующие вашего API‑ключа, а `PublicApi` — методы, которые можно вызывать напрямую из браузера/мобильного устройства и т.д. без аутентификации. `ModerationApi` предоставляет обширный набор живых и быстрых модерационных API. Каждый метод `ModerationApi` принимает параметр `sso` и может аутентифицироваться через SSO или cookie сессии FastComments.com.