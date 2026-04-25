Все настройки находятся в `Administration > Configuration > Content > FastComments` (`/admin/config/content/fastcomments`).

## Обязательно

- **Tenant ID** - Ваш FastComments Tenant ID. Найдите его в [Settings > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).
- **API Secret** - Требуется для Secure SSO, проверки webhook и синхронизации страниц. Находится в [Settings > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).

## Стиль комментариев

Выберите виджет, соответствующий тому, как вы хотите, чтобы пользователи общались на вашем сайте.

- **Live Comments** - Комментарии в реальном времени с поддержкой ветвления.
- **Streaming Chat** - Интерфейс чата в реальном времени, подходит для мероприятий и прямых трансляций.
- **Collab Chat** - Аннотации при выделении текста в основном содержимом. Посетители выделяют текст и начинают обсуждение в контексте.
- **Collab Chat + Comments** - И collab chat, и стандартные комментарии на одной странице.

## Режим SSO

- **None** - Без SSO. Пользователи комментируют как гости или создают аккаунт FastComments.
- **Simple** - Передаёт информацию о пользователе Drupal (name, email, avatar) в FastComments без проверок на сервере.
- **Secure** - Использует HMAC-SHA256 для проверки пользователей Drupal с помощью FastComments. Рекомендуется, если у вас настроен API Secret.

Подробнее смотрите в разделе `Single Sign-On (SSO)`.

## Другие настройки

- **CDN URL** - По умолчанию `https://cdn.fastcomments.com`.
- **Site URL** - По умолчанию `https://fastcomments.com`.
- **Email notifications** - Отправлять уведомление по электронной почте автору контента при появлении нового комментария к его материалу.

По вопросам проживания данных в ЕС см. раздел `EU Data Residency`.

---