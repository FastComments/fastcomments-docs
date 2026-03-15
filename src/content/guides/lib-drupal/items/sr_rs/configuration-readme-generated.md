Идите на **Администрација > Конфигурација > Садржај > FastComments** (`/admin/config/content/fastcomments`).

### Подешавања

- **Tenant ID** (обавезно) - Ваш FastComments Tenant ID. Пронађите га у [Подешавања > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).
- **API Secret** - Потребан за Secure SSO, верификацију webhook-а и синхронизацију страница. Налази се у [Подешавања > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).
- **SSO Mode** - Интеграција Single Sign-On:
  - **None** - Нема SSO; корисници коментаришу као гости или креирају FastComments налоге.
  - **Simple** - Prosleđuje информације о Drupal кориснику (име, имејл, аватар) FastComments-у без верификације на страни сервера.
  - **Secure** - Користи HMAC-SHA256 верификацију за сигурну аутентификацију Drupal корисника са FastComments-ом (препоручено).
- **Commenting Style** - Тип видгета који ће се приказати:
  - **Live Comments** - Коментари у реалном времену са нитном структуром.
  - **Streaming Chat** - Интерфејс за ћаскање у реалном времену.
  - **Collab Chat** - Колаборативне анотације избора текста у главном делу садржаја.
  - **Collab Chat + Comments** - И колаб разговор и стандардни коментари.
- **CDN URL** - FastComments CDN URL (подразумевано: `https://cdn.fastcomments.com`).
- **Site URL** - FastComments URL сајта (подразумевано: `https://fastcomments.com`).
- **Email notifications** - Пошаљите е-поруку ауторима садржаја када је на њиховом садржају објављен нови коментар.

### Додавање коментара у типове садржаја

Додајте поље **FastComments** у своје типове садржаја преко **Структура > Типови садржаја > [type] > Управљање пољима**. Поље има прекидач статуса и опционални прилагођени идентификатор по ентитету.

### Резиденција података у ЕУ

За резиденцију података у ЕУ, ажурирајте:
- **CDN URL** на `https://cdn-eu.fastcomments.com`
- **Site URL** на `https://eu.fastcomments.com`