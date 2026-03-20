Перейдите в **Administration > Configuration > Content > FastComments** (`/admin/config/content/fastcomments`).

### Настройки

- **Tenant ID** (обязательно) - Ваш FastComments Tenant ID. Найдите это в [Настройки > API/SSO](https://fastcomments.com/auth/my-account/api) ([ЕС](https://eu.fastcomments.com/auth/my-account/api)).
- **API Secret** - Требуется для Secure SSO, проверки вебхуков и синхронизации страниц. Находится в [Настройки > API/SSO](https://fastcomments.com/auth/my-account/api) ([ЕС](https://eu.fastcomments.com/auth/my-account/api)).
- **SSO Mode** - Интеграция Single Sign-On:
  - **None** - Без SSO, пользователи комментируют как гости или создают аккаунты FastComments.
  - **Simple** - Передаёт информацию о пользователе Drupal (имя, email, аватар) в FastComments без проверки на стороне сервера.
  - **Secure** - Использует HMAC-SHA256 для безопасной аутентификации пользователей Drupal в FastComments (рекомендуется).
- **Commenting Style** - Тип виджета для отображения:
  - **Live Comments** - Комментарии в реальном времени с ветвлением.
  - **Streaming Chat** - Потоковый чат.
  - **Collab Chat** - Совместная аннотация текста (выделение) в основной области контента.
  - **Collab Chat + Comments** - И совместный чат, и стандартные комментарии.
- **CDN URL** - URL CDN FastComments (по умолчанию: `https://cdn.fastcomments.com`).
- **Site URL** - URL сайта FastComments (по умолчанию: `https://fastcomments.com`).
- **Email notifications** - Отправлять письмо авторам контента при появлении нового комментария на их контенте.

### Добавление комментариев к типам содержимого

Добавьте поле **FastComments** к вашим типам содержимого через **Structure > Content types > [type] > Manage fields**. Поле имеет переключатель статуса и необязательный пользовательский идентификатор для каждой сущности.

### Хранение данных в ЕС

Для хранения данных в ЕС обновите:
- **CDN URL** на `https://cdn-eu.fastcomments.com`
- **Site URL** на `https://eu.fastcomments.com`