All settings live under `Administration > Configuration > Content > FastComments` (`/admin/config/content/fastcomments`).

## Требуется

- **Tenant ID** — ваш FastComments Tenant ID. Найдите его в разделе [Settings > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).
- **API Secret** — требуется для Secure SSO, проверки веб‑хуков и синхронизации страниц. Найдите его в разделе [Settings > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).

## Хранение API Secret

Модуль читает API Secret из первого из этих источников, у которого есть значение:

1. **Key module** — установите [Key](https://www.drupal.org/project/key), создайте ключ *Authentication* и выберите его в поле **API Secret key** на форме настроек. Ключ может быть основан на переменной окружения или файле вне веб‑корня.
2. **settings.php** — добавьте `$config['fastcomments.settings']['api_secret'] = 'your-secret';` в `settings.php` или `settings.local.php`. Секрет никогда не попадает в экспортированную конфигурацию.
3. **Форма настроек** — введите его в поле **API Secret**. Это сохраняет значение в `fastcomments.settings`, поэтому оно попадает в экспорт конфигурации. Отчет о статусе Drupal предупреждает, когда этот источник активен.

Форма показывает, какой источник активен. Оставив поле пустым, вы сохраняете сохранённый секрет. Установите флажок **Remove the API Secret stored in configuration**, чтобы удалить его после перемещения секрета в модуль Key или `settings.php`.

## Стиль комментирования

Выберите виджет, соответствующий тому, как вы хотите, чтобы пользователи общались на вашем сайте.

- **Live Comments** — комментарии в реальном времени с древовидной структурой.
- **Streaming Chat** — интерфейс живого чата, подходит для мероприятий и трансляций.
- **Collab Chat** — аннотация выбранного текста в основной области контента. Посетители выделяют текст и начинают обсуждение в контексте.
- **Collab Chat + Comments** — одновременно collab chat и стандартные комментарии на одной странице.

## Режим SSO

- **None** — без SSO. Пользователи комментируют как гости или создают аккаунт FastComments.
- **Simple** — передаёт информацию о пользователе Drupal (имя, email, аватар) в FastComments без серверной проверки.
- **Secure** — использует HMAC-SHA256 для проверки пользователей Drupal в FastComments. Рекомендуется, когда настроен API Secret.

Смотрите раздел `Single Sign-On (SSO)` для подробностей.

## Другие настройки

- **CDN URL** — по умолчанию `https://cdn.fastcomments.com`.
- **Site URL** — по умолчанию `https://fastcomments.com`.
- **Email notifications** — отправлять email автору контента, когда к его материалу добавлен новый комментарий.

Для резидентности данных в ЕС см. раздел `EU Data Residency`.