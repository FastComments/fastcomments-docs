Сва подешавања се налазе под `Administration > Configuration > Content > FastComments` (`/admin/config/content/fastcomments`).

## Обавезно

- **Tenant ID** - Ваш FastComments Tenant ID. Пронађите га под [Подешавања > API/SSO](https://fastcomments.com/auth/my-account/api) ([ЕУ](https://eu.fastcomments.com/auth/my-account/api)).
- **API Secret** - Потребан за Secure SSO, верификацију webhook-а и синхронизацију страница. Налази се под [Подешавања > API/SSO](https://fastcomments.com/auth/my-account/api) ([ЕУ](https://eu.fastcomments.com/auth/my-account/api)).

## Стил коментарисања

Изаберите видгет који одговара начину на који желите да људи комуницирају на вашем сајту.

- **Live Comments** - У реалном времену, нитовани коментари.
- **Streaming Chat** - Интерфејс за уживо ћаскање, погодан за догађаје и преносе уживо.
- **Collab Chat** - Анотација избора текста у главном делу садржаја. Посетиоци означе текст и покрену дискусију у контексту.
- **Collab Chat + Comments** - И колаб ћаскање и стандардни коментари на истој страници.

## SSO режим

- **None** - Без SSO. Корисници коментаришу као гости или креирају FastComments налог.
- **Simple** - Прослеђује информације о Drupal кориснику (име, имејл, аватар) FastComments-у без верификације на серверу.
- **Secure** - Користи HMAC-SHA256 за верификацију Drupal корисника са FastComments. Препоручује се када имате конфигурисан API Secret.

Погледајте одељак `Single Sign-On (SSO)` за детаље.

## Остала подешавања

- **CDN URL** - Подразумевано је `https://cdn.fastcomments.com`.
- **Site URL** - Подразумевано је `https://fastcomments.com`.
- **Email notifications** - Шаље имејл аутору садржаја када је на њиховом садржају објављен нови коментар.

За резиденцију података у ЕУ, погледајте одељак `EU Data Residency`.

---