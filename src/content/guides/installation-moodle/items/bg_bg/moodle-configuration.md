Страницата с настройки на плъгина се намира в **Site Administration > Plugins > Local plugins > FastComments**. Наличните опции са:

#### Tenant ID

Вашият FastComments Tenant ID. Намерете го в <a href="https://fastcomments.com/auth/my-account" target="_blank">FastComments dashboard</a> в настройките на вашия акаунт.

#### API Secret

Вашият API Secret ключ, необходим за Secure SSO режим. Намерете го в <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">My Account > API Secret</a>.

#### SSO Mode

Изберете как се удостоверяват потребителите. Вижте раздела [SSO Modes](#moodle-sso-modes) за подробности относно всяка опция.

- **Secure** (препоръчително) - сървърно удостоверяване, подписано с HMAC-SHA256
- **Simple** - клиентски данни за потребителя без подпис
- **None** - анонимно коментиране, без интеграция с вход в Moodle

#### Page Contexts

Контролира къде се показват коментарите:

- **Course pages** - коментари на основните страници на курса
- **Module/activity pages** - коментари на отделни дейности и ресурси
- **Both** - коментари на всички типове страници

#### Commenting Style

Изберете начина на коментиране. Вижте [Commenting Styles](#moodle-commenting-styles) за екранни снимки на всеки режим.

- **Comments** - стандартен нишков коментарен модул под съдържанието на страницата
- **Collab Chat** - инлайн дискусии чрез селекция на текст с индикатори за присъствие
- **Both** - коментари и collab chat активни едновременно

#### CDN URL

URL на FastComments CDN. По подразбиране `https://cdn.fastcomments.com`. Променете това към EU CDN URL, ако вашите данни се хостват в региона на ЕС.