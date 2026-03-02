Страницата с настройки на плъгина е в **Администрация на сайта > Плъгини > Локални плъгини > FastComments**. Наличните опции са:

#### Tenant ID

Вашият Tenant ID на FastComments. Намерете го в <a href="https://fastcomments.com/auth/my-account" target="_blank">таблото на FastComments</a> в настройките на вашия акаунт.

#### API Secret

Вашият API Secret ключ, необходим за режим Secure SSO. Намерете го в <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">Моят акаунт > API Secret</a>.

#### SSO Mode

Изберете как потребителите се удостоверяват. Вижте раздела [SSO Modes](#items-moodle-sso-modes) за подробности за всяка опция.

- **Secure** (препоръчва се) - сървърно подписано удостоверяване HMAC-SHA256
- **Simple** - клиентски потребителски данни без подпис
- **None** - анонимно коментиране, без интеграция с Moodle влизане

#### Page Contexts

Контролира къде се появяват коментарите:

- **Course pages** - коментари на главните страници на курса
- **Module/activity pages** - коментари на отделните дейности и ресурси
- **Both** - коментари на всички типове страници

#### Commenting Style

Изберете преживяването при коментиране. Вижте [Commenting Styles](#items-moodle-commenting-styles) за екранни снимки на всеки режим.

- **Comments** - стандартен разклонен коментарен уиджет под съдържанието на страницата
- **Collab Chat** - обсъждания чрез избор на текст в мястото с индикатори за присъствие
- **Both** - коментари и Collab Chat активни заедно

#### CDN URL

URL на CDN на FastComments. По подразбиране е `https://cdn.fastcomments.com`. Променете това на EU CDN URL, ако вашите данни се хостват в региона на ЕС.