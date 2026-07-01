FastComments SDK предоставляет три клиента API:

### PublicAPI - Client-Safe Methods

`PublicAPI` содержит методы, безопасные для вызова из клиентского кода (iOS/macOS‑приложения). Эти методы:
- Не требуют API‑ключа
- Может использовать токены SSO для аутентификации
- Ограничены по скорости для каждого пользователя/устройства
- Подходят для приложений, ориентированных на конечных пользователей

**Example use case**: Fetching and creating comments in your iOS app

### DefaultAPI - Server-Side Methods

`DefaultAPI` содержит аутентифицированные методы, которые требуют API‑ключ. Эти методы:
- Требует ваш FastComments API‑ключ
- Должен вызываться ТОЛЬКО из серверного кода
- Обеспечивает полный доступ к вашим данным FastComments
- Ограничены по скорости для каждого арендатора

**Example use case**: Administrative operations, bulk data export, user management

### ModerationAPI - Moderator Dashboard Methods

`ModerationAPI` предоставляет обширный набор живых и быстрых API модерации. Каждый метод `ModerationAPI` принимает параметр `sso` и может аутентифицироваться через SSO или cookie‑сессию FastComments.com.

**Example use case**: Building a moderation experience for moderators of your community

**IMPORTANT**: Never expose your API key in client-side code. API keys should only be used server-side.