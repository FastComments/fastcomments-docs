Страница подешавања додатка је на **Site Administration > Plugins > Local plugins > FastComments**. Доступне опције су:

#### ID zakupca

Ваш FastComments Tenant ID. Пронађите ово у <a href="https://fastcomments.com/auth/my-account" target="_blank">FastComments контролној табли</a> у подешавањима налога.

#### API тајни кључ

Ваш API Secret кључ, потребан за Secure SSO режим. Пронађите ово на <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">My Account > API Secret</a>.

#### SSO режим

Изаберите како се корисници аутентификују. Погледајте секцију [SSO Modes](#moodle-sso-modes) за детаље о свакој опцији.

- **Sigurno** (препоручено) - HMAC-SHA256 потписана аутентификација на serverskoj strani
- **Jednostavno** - подаци о кориснику на strani клијента без потписа
- **Nijedno** - анонимно коментарисање, без интеграције Moodle пријаве

#### Контеkсти страница

Контролише где се коментари појављују:

- **Странице курса** - коментари на главним страницама курса
- **Странице модула/активности** - коментари на појединачним активностима и ресурсима
- **Обоје** - коментари на свим типовима страница

#### Стил коментарисања

Изаберите искуство коментарисања. Погледајте [Stilovi komentarisanja](#moodle-commenting-styles) за снимке екрана сваког режима.

- **Коментари** - стандардни униткнути (threaded) видгет за коментаре испод садржаја странице
- **Collab Chat** - дискусије у линији на основу избора текста са индикаторима присутности
- **Обоје** - коментари и collab chat активни заједно

#### CDN URL

FastComments CDN URL. Подразумевано је `https://cdn.fastcomments.com`. Промените ово на EU CDN URL ако су ваши подаци хостовани у ЕУ региону.