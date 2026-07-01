### Установить с GitHub

Установите напрямую из тега релиза (рекомендовано, полностью воспроизводимо):

```bash
pip install git+https://github.com/fastcomments/fastcomments-python.git@v3.0.0
```

Привязывайте тег, а не ветку, чтобы сборки были детерминированными. Тот же формат работает в `requirements.txt`:

```
fastcomments @ git+https://github.com/fastcomments/fastcomments-python.git@v3.0.0
```

Каждый помеченный [GitHub Release](https://github.com/fastcomments/fastcomments-python/releases) также имеет прикреплённый готовый wheel, если вы предпочитаете установить бинарный артефакт напрямую.

### Содержимое библиотеки

Эта библиотека содержит два модуля: сгенерированный клиент API и основную Python‑библиотеку, в которой находятся написанные вручную утилиты, упрощающие работу с API, включая поддержку SSO.

- [Документация клиента API](https://github.com/FastComments/fastcomments-python/blob/main/client/README.md)
- [Документация основной библиотеки, включая примеры SSO](https://github.com/FastComments/fastcomments-python/blob/main/sso/README.md)

### Публичные и защищённые API

Для клиента API существуют три класса: `DefaultApi`, `PublicApi` и `ModerationApi`. `DefaultApi` содержит методы, требующие вашего API‑ключа, а `PublicApi` — методы, которые можно вызывать напрямую из браузера, мобильного устройства и т.п. без аутентификации. `ModerationApi` предоставляет обширный набор быстрых и живых модерационных API. Каждый метод `ModerationApi` принимает параметр `sso` и может аутентифицироваться через SSO или cookies сеанса FastComments.com.