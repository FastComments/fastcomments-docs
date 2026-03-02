Страница са подешавањима плагина је у **Site Administration > Plugins > Local plugins > FastComments**. Доступне опције су:

#### Tenant ID

Ваш FastComments Tenant ID. Пронађите га у <a href="https://fastcomments.com/auth/my-account" target="_blank">FastComments dashboard</a> у подешавањима налога.

#### API Secret

Ваш API Secret кључ, потребан за Secure SSO режим. Пронађите га на <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">My Account > API Secret</a>.

#### SSO Mode

Изаберите како се корисници аутентификују. Погледајте [SSO Modes](#moodle-sso-modes) одељак за детаље о свакој опцији.

- **Secure** (препоручено) - серверска аутентификација потписана HMAC-SHA256
- **Simple** - подаци о кориснику на клијентској страни без потписа
- **None** - анонимно коментарисање, нема интеграције са Moodle пријавом

#### Page Contexts

Контролише где се коментари појављују:

- **Course pages** - коментари на главним страницама курса
- **Module/activity pages** - коментари на појединачним активностима и ресурсима
- **Both** - коментари на свим типовима страница

#### Commenting Style

Изаберите искуство коментарисања. Погледајте [Commenting Styles](#moodle-commenting-styles) за снимке екрана сваког режима.

- **Comments** - стандардни видгет за нити коментара испод садржаја странице
- **Collab Chat** - инлајн дискусије избором текста са индикаторима присуства
- **Both** - коментари и Collab Chat активни заједно

#### CDN URL

The FastComments CDN URL. Подразумевано је `https://cdn.fastcomments.com`. Промените ово у EU CDN URL ако се ваши подаци хостују у ЕУ регији.