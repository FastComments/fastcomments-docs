Идите на **Administration > Configuration > Content > FastComments** (`/admin/config/content/fastcomments`).

### Подешавања

- **Tenant ID** (обавезно) - Ваш FastComments Tenant ID. Пронађите ово у [Подешавања > API/SSO](https://fastcomments.com/auth/my-account/api) ([ЕУ](https://eu.fastcomments.com/auth/my-account/api)).
- **API Secret** - Потребно за Secure SSO, верификацију webhook-а и синхронизацију страница. Налази се у [Подешавања > API/SSO](https://fastcomments.com/auth/my-account/api) ([ЕУ](https://eu.fastcomments.com/auth/my-account/api)).
- **SSO Mode** - Интеграција Single Sign-On:
  - **None** - Без SSO-а, корисници коментаришу као гости или креирају FastComments налоге.
  - **Simple** - Прослеђује Drupal информације о кориснику (име, имејл, аватар) ка FastComments без серверске верификације.
  - **Secure** - Користи HMAC-SHA256 верификацију за сигурну аутентикацију Drupal корисника са FastComments (препоручено).
- **Commenting Style** - Тип виџета који ће бити приказан:
  - **Live Comments** - Реално-временски нитовани коментари.
  - **Streaming Chat** - Интерфејс за уживо чет.
  - **Collab Chat** - Сарадничко означавање текста на главном садржају.
  - **Collab Chat + Comments** - И collab chat и стандардни коментари.
- **CDN URL** - FastComments CDN URL (подразумевано: `https://cdn.fastcomments.com`).
- **Site URL** - FastComments Site URL (подразумевано: `https://fastcomments.com`).
- **Email notifications** - Пошаљи имејл ауторима садржаја када је нови коментар објављен на њиховом садржају.

### Додавање коментара у типове садржаја

Додајте поље **FastComments** у ваше типове садржаја преко **Structure > Content types > [type] > Manage fields**. Поље има прекидач статуса и опциони прилагођени идентификатор по ентитету.

### Резиденција података за ЕУ

За резиденцију података у ЕУ, ажурирајте:
- **CDN URL** на `https://cdn-eu.fastcomments.com`
- **Site URL** на `https://eu.fastcomments.com`