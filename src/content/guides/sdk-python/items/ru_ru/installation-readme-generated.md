### PyPI

```bash
pip install fastcomments
```

### Содержимое библиотеки

Эта библиотека содержит два модуля: сгенерированный API-клиент и основная Python-библиотека, содержащая вручную написанные утилиты, упрощающие работу с API, включая поддержку SSO.

- [Документация клиентской библиотеки API](https://github.com/FastComments/fastcomments-python/blob/main/client/README.md)
- [Документация основной библиотеки, включая примеры SSO](https://github.com/FastComments/fastcomments-python/blob/main/sso/README.md)

### Публичные и защищённые API

В API-клиенте имеются три класса: `DefaultApi`, `PublicApi` и `ModerationApi`. Класс `DefaultApi` содержит методы, требующие вашего API-ключа, а `PublicApi` — методы, которые можно вызывать напрямую из браузера/мобильного устройства и т.п. без аутентификации. Класс `ModerationApi` отвечает за панель модератора и содержит методы для модерации комментариев (list, count, search, logs, export), действий модерации (remove/restore, flag, set review/spam/approval status, votes, reopen/close thread), банов (ban from comment, undo, pre-ban summaries, ban status/preferences, banned-user counts), а также для значков и доверия (award/remove badge, manual badges, get/set trust factor, user internal profile). Каждый метод `ModerationApi` принимает параметр `sso`, чтобы его можно было вызывать от имени модератора, аутентифицированного через SSO.