Страница настроек плагина находится в **Администрирование сайта > Плагины > Локальные плагины > FastComments**. Доступные параметры:

#### Tenant ID

Ваш Tenant ID FastComments. Найдите его в <a href="https://fastcomments.com/auth/my-account" target="_blank">панели управления FastComments</a> в настройках аккаунта.

#### API Secret

Ваш ключ API Secret, необходимый для режима Secure SSO. Найдите его в <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">Мой аккаунт > API Secret</a>.

#### SSO Mode

Выберите способ аутентификации пользователей. См. раздел [Режимы SSO](#moodle-sso-modes) для подробностей по каждому варианту.

- **Secure** (рекомендуется) - серверная подпись аутентификации HMAC-SHA256
- **Simple** - клиентские данные пользователя без подписи
- **None** - анонимные комментарии, без интеграции входа Moodle

#### Page Contexts

Управляет местом отображения комментариев:

- **Course pages** - комментарии на главных страницах курса
- **Module/activity pages** - комментарии к отдельным активностям и ресурсам
- **Both** - комментарии на всех типах страниц

#### Commenting Style

Выберите стиль комментариев. См. [Commenting Styles](#moodle-commenting-styles) для скриншотов каждого режима.

- **Comments** - стандартный древовидный виджет комментариев под содержимым страницы
- **Collab Chat** - встроенные обсуждения по выделению текста с индикаторами присутствия
- **Both** - одновременно активны и комментарии, и Collab Chat

#### CDN URL

URL CDN FastComments. По умолчанию `https://cdn.fastcomments.com`. Измените это на EU CDN URL, если ваши данные размещены в регионе ЕС.