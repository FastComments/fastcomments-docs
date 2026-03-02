Страница настроек плагина находится в **Администрирование сайта > Плагины > Локальные плагины > FastComments**. Доступные параметры:

#### Tenant ID

Ваш FastComments Tenant ID. Найдите это в <a href="https://fastcomments.com/auth/my-account" target="_blank">панели управления FastComments</a> в настройках аккаунта.

#### API Secret

Ваш API Secret ключ, необходимый для режима Secure SSO. Найдите его в <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">My Account > API Secret</a>.

#### SSO Mode

Выберите, как аутентифицируются пользователи. Смотрите раздел [Режимы SSO](#items-moodle-sso-modes) для подробностей по каждому варианту.

- **Secure** (recommended) - серверная аутентификация с подписью HMAC-SHA256
- **Simple** - данные пользователя на стороне клиента без подписи
- **None** - анонимные комментарии, без интеграции входа в Moodle

#### Page Contexts

Управляет местом отображения комментариев:

- **Course pages** - комментарии на главных страницах курса
- **Module/activity pages** - комментарии на отдельных активностях и ресурсах
- **Both** - комментарии на всех типах страниц

#### Commenting Style

Выберите опыт взаимодействия с комментариями. Смотрите раздел [Стили комментирования](#items-moodle-commenting-styles) для скриншотов каждого режима.

- **Comments** - стандартный древовидный виджет комментариев под содержимым страницы
- **Collab Chat** - обсуждения, инициируемые выделением текста, с индикаторами присутствия
- **Both** - комментарии и Collab Chat активны одновременно

#### CDN URL

URL CDN FastComments. По умолчанию `https://cdn.fastcomments.com`. Измените это на EU CDN URL, если ваши данные размещены в регионе ЕС.