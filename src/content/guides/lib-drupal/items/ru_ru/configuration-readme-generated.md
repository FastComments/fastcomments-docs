Перейдите в **Administration > Configuration > Content > FastComments** (`/admin/config/content/fastcomments`).

### Настройки

- **Tenant ID** (обязательно) - Ваш FastComments Tenant ID. Найдите это в разделе [Settings > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).
- **API Secret** - Требуется для Secure SSO, webhook verification и синхронизации страниц. Находится в разделе [Settings > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).
- **SSO Mode** - Интеграция Single Sign-On:
  - **None** - Без SSO: пользователи комментируют как гости или создают аккаунты FastComments.
  - **Simple** - Передаёт информацию пользователя Drupal (name, email, avatar) в FastComments без проверки на стороне сервера.
  - **Secure** - Использует HMAC-SHA256 для проверки и безопасной аутентификации пользователей Drupal в FastComments (рекомендуется).
- **Commenting Style** - Тип виджета для отображения:
  - **Live Comments** - Потоковые ветвящиеся комментарии в реальном времени.
  - **Streaming Chat** - Интерфейс живого чата.
  - **Collab Chat** - Совместная аннотация выделенного текста в основной области контента.
  - **Collab Chat + Comments** - И collab chat, и стандартные комментарии.
- **CDN URL** - FastComments CDN URL (по умолчанию: `https://cdn.fastcomments.com`).
- **Site URL** - FastComments site URL (по умолчанию: `https://fastcomments.com`).
- **Email notifications** - Отправлять электронное письмо авторам контента, когда на их материал добавлен новый комментарий.

### Добавление комментариев к типам контента

Добавьте поле **FastComments** к вашим типам контента через **Structure > Content types > [type] > Manage fields**. Поле имеет переключатель статуса и необязательный индивидуальный идентификатор для каждой сущности.

### Размещение данных в ЕС

Для размещения данных в ЕС обновите:
- **CDN URL** на `https://cdn-eu.fastcomments.com`
- **Site URL** на `https://eu.fastcomments.com`