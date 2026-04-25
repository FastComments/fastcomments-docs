Сва подешавања се налазе у `Administration > Configuration > Content > FastComments` (`/admin/config/content/fastcomments`).

## Обавезно

- **Tenant ID** - Ваш FastComments Tenant ID. Пронађите га у [Подешавања > API/SSO](https://fastcomments.com/auth/my-account/api) ([ЕУ](https://eu.fastcomments.com/auth/my-account/api)).
- **API Secret** - Потребан за Secure SSO, верификацију webhook-ова и синхронизацију страница. Налази се у [Подешавања > API/SSO](https://fastcomments.com/auth/my-account/api) ([ЕУ](https://eu.fastcomments.com/auth/my-account/api)).

## Стил коментарисања

Изаберите видгет који одговара начину на који желите да људи комуницирају на вашем сајту.

- **Live Comments** - Коментари у реалном времену, организовани у нити.
- **Streaming Chat** - Уживо чет интерфејс, погодан за догађаје и livestream-ове.
- **Collab Chat** - Анотација селекцијом текста у главном садржају. Посетиоци означе текст и започну дискусију у контексту.
- **Collab Chat + Comments** - И collab chat и стандардни коментари на истој страници.

## SSO режим

- **None** - Нема SSO. Корисници коментаришу као гости или креирају FastComments налог.
- **Simple** - Прослеђује Drupal корисничке податке (име, имејл, аватар) FastComments-у без верификације на серверу.
- **Secure** - Користи HMAC-SHA256 за верификацију Drupal корисника са FastComments-om. Препоручује се када имате конфигурисан API Secret.

Погледајте одељак `Single Sign-On (SSO)` за детаље.

## Остала подешавања

- **CDN URL** - Подразумевано: `https://cdn.fastcomments.com`.
- **Site URL** - Подразумевано: `https://fastcomments.com`.
- **Email notifications** - Шаље е-поруку аутору садржаја када је постављен нов коментар на њихов садржај.

За резидентност података у ЕУ, погледајте одељак `EU Data Residency`.