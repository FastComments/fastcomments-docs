Sva podešavanja se nalaze u `Administration > Configuration > Content > FastComments` (`/admin/config/content/fastcomments`).

## Обавезно

- **Tenant ID** - Ваш FastComments Tenant ID. Проналаже се под [Подешавања > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).
- **API Secret** - Потребно за Secure SSO, верификацију webhook-а и синхронизацију страница. Налази се под [Подешавања > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).

## Стил коментарисања

Изаберите видџет који одговара начину на који желите да људи комуницирају на вашем сајту.

- **Live Comments** - Коментари у реалном времену са уткнутим нитима.
- **Streaming Chat** - Интерфејс за ћаскање уживо, погодан за догађаје и преносе уживо.
- **Collab Chat** - Анотације одабиром текста у главном садржају. Посетиоци означе текст и покрену дискусију у контексту.
- **Collab Chat + Comments** - И Collab Chat и стандардни коментари на истој страници.

## SSO режим

- **None** - Нема SSO. Корисници коментаришу као гости или направе FastComments налог.
- **Simple** - Прослеђује Drupal информације о кориснику (name, email, avatar) FastComments без верификације на serverskoj strani.
- **Secure** - Користи HMAC-SHA256 да верификује Drupal кориснике са FastComments. Препоручено када имате конфигурисани API Secret.

Погледајте оделjak `Single Sign-On (SSO)` за детаље.

## Остала подешавања

- **CDN URL** - Подразумевано је `https://cdn.fastcomments.com`.
- **Site URL** - Подразумевано је `https://fastcomments.com`.
- **Email notifications** - Шаље е-поруку аутору садржаја када је објављен нови коментар на њиховом садржају.

За пребивалиште података у ЕУ, погледајте одeljak `EU Data Residency`.