### PyPI

```bash
pip install fastcomments
```

### Содержимое библиотеки

Эта библиотека содержит два модуля: сгенерированный клиент API и основную библиотеку Python, которая содержит написанные вручную утилиты для упрощения работы с API, включая поддержку SSO.

- [Документация клиентской библиотеки API](https://github.com/FastComments/fastcomments-python/blob/main/client/README.md)
- [Документация основной библиотеки, включая примеры SSO](https://github.com/FastComments/fastcomments-python/blob/main/sso/README.md)

### Публичные и защищённые API

Для клиентской части API есть три класса, `DefaultApi`, `PublicApi`, и `ModerationApi`. `DefaultApi` содержит методы, которые требуют ваш API-ключ, а `PublicApi` содержит методы, которые можно вызывать непосредственно из браузера, мобильного устройства и т. п. без аутентификации. `ModerationApi` обеспечивает работу панели модератора и содержит методы для модерации комментариев (list, count, search, logs, export), действия модерации (remove/restore, flag, set review/spam/approval status, votes, reopen/close thread), баны (ban from comment, undo, pre-ban summaries, ban status/preferences, banned-user counts) и бейджи & доверие (award/remove badge, manual badges, get/set trust factor, user internal profile). Каждый метод `ModerationApi` принимает параметр `sso`, чтобы его можно было вызвать от имени модератора, аутентифицированного через SSO.