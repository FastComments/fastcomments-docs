Страница поставки додатка је на **Site Administration > Plugins > Local plugins > FastComments**. Доступне опције су:

#### Tenant ID

Ваш FastComments Tenant ID. Пронађите ово на <a href="https://fastcomments.com/auth/my-account" target="_blank">FastComments контролној табли</a> у оквиру подешавања налога.

#### API Secret

Ваш API тајни кључ, потребан за Secure SSO режим. Пронађите ово на <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">Мој налог > API тајна</a>.

#### SSO Mode

Изаберите како се корисници аутентификују. Погледајте одељак [SSO режими](#items-moodle-sso-modes) за детаље о свакој опцији.

- **Secure** (препоручено) - серверска HMAC-SHA256 потписана аутентификација
- **Simple** - подаци о кориснику на клијентској страни без потписа
- **None** - анонимно коментарисање, без интеграције Moodle пријаве

#### Page Contexts

Контролише где се коментари појављују:

- **Course pages** - коментари на главним страницама курса
- **Module/activity pages** - коментари на појединачним активностима и ресурсима
- **Both** - коментари на свим типовима страница

#### Commenting Style

Изаберите искуство коментарисања. Погледајте [Стилове коментарисања](#items-moodle-commenting-styles) за скриншотове сваког режима.

- **Comments** - стандардни уграђени threaded коментар испод садржаја странице
- **Collab Chat** - дискусије унутар текста изабраног као inline са индикаторима присутности
- **Both** - коментари и collab chat активни заједно

#### CDN URL

FastComments CDN URL. Подразумева се на `https://cdn.fastcomments.com`. Промените ово на EU CDN URL ако су ваши подаци хостовани у EU региону.