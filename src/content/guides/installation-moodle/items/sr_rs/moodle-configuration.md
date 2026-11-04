Страница са подешавањима додатка налази се на **Site Administration > Plugins > Local plugins > FastComments**. Доступне опције су:

#### Tenant ID

Ваш FastComments Tenant ID. Пронађите га у <a href="https://fastcomments.com/auth/my-account" target="_blank">FastComments контролном панелу</a> у подешавањима свог налога.

#### API Secret

Ваш API Secret кључ, неопходан за Secure SSO режим. Пронађите га на <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">My Account > API Secret</a>.

#### SSO Mode

Изаберите начин на који се корисници аутентификују. Погледајте одељак [SSO Modes](#moodle-sso-modes) за детаље о свакој опцији.

- **Secure** (препоручено) - серверска аутентификација потписана HMAC-SHA256
- **Simple** - подаци о кориснику на страни клијента без потписа
- **None** - анонимно коментарисање, без интеграције са Moodle пријавом

#### Page Contexts

Контролише где се коментари појављују:

- **Course pages** - коментари на главним страницама курса
- **Module/activity pages** - коментари на појединачним активностима и ресурсима
- **Both** - коментари на свим типовима страница

#### Commenting Style

Изаберите искуство коментарисања. Погледајте [Commenting Styles](#moodle-commenting-styles) за снимке екрана сваког режима.

- **Comments** - стандардни виџет са грананим коментарима испод садржаја странице
- **Collab Chat** - дискусије у линији око изабраног текста са индикаторима присуства
- **Both** - коментари и колаборативни ћаскање активни заједно

#### CDN URL

FastComments CDN URL. Подразумевано је `https://cdn.fastcomments.com`. Промените ово на EU CDN URL ако су ваши подаци хостовани у EU региону.