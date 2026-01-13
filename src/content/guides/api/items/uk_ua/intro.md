### API FastComments

FastComments надає API для взаємодії з багатьма ресурсами. Створюйте інтеграції з нашою платформою або навіть пишіть власні клієнти!

У цій документації ви знайдете всі ресурси, що підтримуються API, задокументовані разом із типами запитів і відповідей.

Для корпоративних (Enterprise) клієнтів увесь доступ до API фіксується в журналі аудиту.

### Згенеровані SDK

FastComments тепер генерує [API Spec](https://fastcomments.com/js/swagger.json) з нашого коду (це ще не завершено, але включає багато API).

Також тепер у нас є SDK для популярних мов:

- [fastcomments-cpp](./guide-sdk-cpp.html)
- [fastcomments-go](./guide-sdk-go.html)
- [fastcomments-java](./guide-sdk-java.html)
- [fastcomments-sdk-js](./guide-sdk-javascript.html)
- [fastcomments-nim](./guide-sdk-nim.html)
- [fastcomments-php](guide-sdk-php.html)
- [fastcomments-php-sso](./guide-sdk-php-sso.html)
- [fastcomments-python](./guide-sdk-python.html)
- [fastcomments-ruby](./guide-sdk-ruby.html)
- [fastcomments-rust](./guide-sdk-rust.html)
- [fastcomments-swift](./guide-sdk-swift.html)

### Аутентифікація

Доступ до API аутентифікується шляхом передачі вашого [api key](https://fastcomments.com/auth/my-account/api-secret) у вигляді заголовка `X-API-KEY` або як параметра запиту `API_KEY`. Вам також знадобиться ваш `tenantId` для викликів API. Його можна отримати на тій же сторінці, що й ваш api key.

### Примітка щодо безпеки

Ці маршрути призначені для виклику з **серверу**. __НЕ__ викликайте їх з браузера. Це розкриє ваш API-ключ — це надасть повний доступ до вашого облікового запису будь-кому, хто може переглянути вихідний код сторінки!

#### Варіант аутентифікації 1 — Заголовки

- Заголовок: `X-API-KEY`
- Заголовок: `X-TENANT-ID`

#### Варіант аутентифікації 2 — Параметри запиту

- Параметр запиту: `API_KEY`
- Параметр запиту: `tenantId`

---