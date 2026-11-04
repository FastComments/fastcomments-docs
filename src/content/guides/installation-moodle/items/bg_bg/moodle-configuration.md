Страницата с настройки на плъгина се намира на **Site Administration > Plugins > Local plugins > FastComments**. Наличните опции са:

#### Tenant ID

Вашият FastComments Tenant ID. Намерете го в <a href="https://fastcomments.com/auth/my-account" target="_blank">FastComments таблото</a> под настройките на акаунта си.

#### API Secret

Вашият API Secret ключ, необходим за режим Secure SSO. Намерете го на <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">My Account > API Secret</a>.

#### SSO Mode

Изберете как потребителите се удостоверяват. Вижте раздела [SSO Modes](#moodle-sso-modes) за подробности за всяка опция.

- **Secure** (препоръчително) - удостоверяване от страна на сървъра с HMAC-SHA256 подпис
- **Simple** - потребителски данни от страна на клиента без подпис
- **None** - коментиране като анонимен потребител, без интеграция с Moodle вход

#### Page Contexts

Контролира къде се показват коментарите:

- **Course pages** - коментари на основните страници на курсовете
- **Module/activity pages** - коментари на отделни дейности и ресурси
- **Both** - коментари на всички типове страници

#### Commenting Style

Изберете изживяването при коментиране. Вижте [Commenting Styles](#moodle-commenting-styles) за екранни снимки на всеки режим.

- **Comments** - стандартен нишкови коментари уиджет под съдържанието на страницата
- **Collab Chat** - дискусии чрез маркиране на текст с индикатори за присъствие
- **Both** - коментари и collab chat активни едновременно

#### CDN URL

FastComments CDN URL. По подразбиране е `https://cdn.fastcomments.com`. Променете го на EU CDN URL, ако данните ви са хоствани в EU региона.