Navигирајте до **Администрација > Конфигурација > Садржај > FastComments** (`/admin/config/content/fastcomments`).

### Подешавања

- **Tenant ID** (обавезно) - Ваш FastComments Tenant ID. Пронађите ово под [Подешавања > API/SSO](https://fastcomments.com/auth/my-account/api) ([ЕУ](https://eu.fastcomments.com/auth/my-account/api)).
- **API Secret** - Потребно за Secure SSO, верификацију webhook-а и синхронизацију страница. Пронађено под [Подешавања > API/SSO](https://fastcomments.com/auth/my-account/api) ([ЕУ](https://eu.fastcomments.com/auth/my-account/api)).
- **SSO Mode** - Single Sign-On интеграција:
  - **None** - Нема SSO, корисници коментаришу као гости или креирају FastComments налоге.
  - **Simple** - Прослеђује информације о Drupal кориснику (име, е-пошта, аватар) у FastComments без серверске верификације.
  - **Secure** - Користи HMAC-SHA256 верификацију да сигурно аутентификује Drupal кориснике са FastComments (препоручено).
- **Commenting Style** - Врста видгета која ће бити приказана:
  - **Live Comments** - Коментари у реалном времену са нитима.
  - **Streaming Chat** - Интерфејс за ливе чет.
  - **Collab Chat** - Колаборативна анотација избора текста на главном садржају.
  - **Collab Chat + Comments** - И колаб чат и стандардни коментари.
- **CDN URL** - FastComments CDN URL (подразумевано: `https://cdn.fastcomments.com`).
- **Site URL** - FastComments site URL (подразумевано: `https://fastcomments.com`).
- **Email notifications** - Слање е-поште ауторима садржаја када се на њиховом садржају појави нови коментар.

### Додавање коментара у типове садржаја

Додајте поље **FastComments** у своје типове садржаја преко **Структура > Типови садржаја > [type] > Управљање пољима**. Поље има прекидач статуса и опциони прилагођени идентификатор по ентитету.

### Резиденција података у ЕУ

За резиденцију података у ЕУ, ажурирајте:
- **CDN URL** на `https://cdn-eu.fastcomments.com`
- **Site URL** на `https://eu.fastcomments.com`