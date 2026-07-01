### Install from GitHub

Установите напрямую из тега релиза (рекомендовано, полностью воспроизводимо):

```bash
pip install git+https://github.com/fastcomments/fastcomments-python.git@v3.0.0
```

Закрепите тег, а не ветку, чтобы сборки были детерминированными. Тот же синтаксис работает в `requirements.txt`:

```
fastcomments @ git+https://github.com/fastcomments/fastcomments-python.git@v3.0.0
```

Каждый помеченный [GitHub Release](https://github.com/fastcomments/fastcomments-python/releases) также имеет прикреплённый построенный wheel, если вы предпочитаете установить бинарный артефакт напрямую.

### Library Contents

Эта библиотека содержит два модуля: сгенерированный API‑клиент и ядро Python‑библиотеки, которое включает написанные вручную утилиты для упрощения работы с API, включая поддержку SSO.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-python/blob/main/client/README.md)
- [Core Library Docs, Including SSO Examples](https://github.com/FastComments/fastcomments-python/blob/main/sso/README.md)

### Public vs Secured APIs

Для API‑клиента существует три класса: `DefaultApi`, `PublicApi` и `ModerationApi`. `DefaultApi` содержит методы, требующие вашего API‑ключа, а `PublicApi` — методы, которые можно вызывать напрямую из браузера/мобильного устройства и т.д. без аутентификации. `ModerationApi` предоставляет обширный набор живых и быстрых API модерации. Каждый метод `ModerationApi` принимает параметр `sso` и может аутентифицироваться через SSO или cookie сессии FastComments.com.