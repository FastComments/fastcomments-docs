Все настройки находятся в разделе `Administration > Configuration > Content > FastComments` (`/admin/config/content/fastcomments`).

## Обязательные

- **Tenant ID** - Ваш FastComments Tenant ID. Найдите его в разделе [Settings > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).
- **API Secret** - Требуется для Secure SSO, проверки вебхуков и синхронизации страниц. Находится в разделе [Settings > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).

## Стиль комментирования

Выберите виджет, который соответствует тому, как вы хотите, чтобы посетители общались на вашем сайте.

- **Live Comments** - Древовидные комментарии в реальном времени.
- **Streaming Chat** - Интерфейс живого чата, подходит для мероприятий и прямых трансляций.
- **Collab Chat** - Аннотации по выделению текста в основном контенте. Посетители выделяют текст и начинают обсуждение в контексте.
- **Collab Chat + Comments** - И collab chat, и стандартные комментарии на одной странице.

## Режим SSO

- **None** - Без SSO. Пользователи комментируют как гости или создают аккаунт FastComments.
- **Simple** - Передаёт информацию пользователя Drupal (name, email, avatar) в FastComments без серверной проверки.
- **Secure** - Использует HMAC-SHA256 для проверки пользователей Drupal с FastComments. Рекомендуется при настроенном API Secret.

См. раздел `Single Sign-On (SSO)` для подробностей.

## Другие настройки

- **CDN URL** - По умолчанию `https://cdn.fastcomments.com`.
- **Site URL** - По умолчанию `https://fastcomments.com`.
- **Email notifications** - Отправлять уведомление по электронной почте автору контента, когда на его материале появляется новый комментарий.

Для размещения данных в ЕС см. раздел `EU Data Residency`.