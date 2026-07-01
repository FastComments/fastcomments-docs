The FastComments SDK provides three API clients:

### PublicAPI - Методи, безпечні для клієнта

The `PublicAPI` contains methods that are safe to call from client-side code (iOS/macOS apps). These methods:
- Не вимагають API‑ключа
- Можуть використовувати SSO‑токени для автентифікації
- Мають обмеження швидкості на користувача/пристрій
- Підходять для застосувань, орієнтованих на кінцевих користувачів

**Приклад використання**: Отримання та створення коментарів у вашому iOS‑додатку

### DefaultAPI - Методи серверної сторони

The `DefaultAPI` contains authenticated methods that require an API key. These methods:
- Потребують вашого API‑ключа FastComments
- Повинні **ЛИШЕ** викликатися з коду серверної сторони
- Надають повний доступ до ваших даних FastComments
- Мають обмеження швидкості на орендаря

**Приклад використання**: Адміністрування, масовий експорт даних, керування користувачами

### ModerationAPI - Методи панелі модератора

The `ModerationAPI` provides an extensive suite of live and fast moderation APIs. Every `ModerationAPI` method accepts an `sso` parameter and can authenticate via SSO or a FastComments.com session cookie.

**Приклад використання**: Створення досвіду модерації для модераторів вашої спільноти

**ВАЖЛИВО**: Ніколи не розкривайте ваш API‑ключ у коді клієнтської сторони. API‑ключі слід використовувати лише на стороні сервера.