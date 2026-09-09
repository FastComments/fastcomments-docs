### FastComments API

FastComments надає API для взаємодії з багатьма ресурсами. Створюйте інтеграції з нашою платформою або навіть створюйте власних клієнтів!

У цій документації ви знайдете всі підтримувані ресурси API, задокументовані з їх типами запитів і відповідей.

Для корпоративних клієнтів весь доступ до API фіксується в журналі аудиту.

### Згенеровані SDK

FastComments тепер генерує [API Spec](https://fastcomments.com/js/swagger.json) з нашого коду (це ще не завершено, але включає багато API).

У нас також є SDK для популярних мов:

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

API аутентифікується шляхом передачі вашого [api key](https://fastcomments.com/auth/my-account/api-secret) у вигляді заголовка `X-API-KEY` або параметра запиту `API_KEY`. Вам також знадобиться ваш `tenantId` для виконання API‑викликів. Його можна отримати на тій же сторінці, що й ваш api key.

### Примітка щодо безпеки

Ці маршрути призначені для виклику з **сервера**. __НЕ ПОВИННІ__ викликати їх з браузера. Це розкриє ваш API‑ключ — це надасть повний доступ до вашого облікового запису будь‑кому, хто зможе переглянути вихідний код сторінки!

#### Варіант аутентифікації 1 — Заголовки

- Header: `X-API-KEY`
- Header: `X-TENANT-ID`

#### Варіант аутентифікації 2 — Параметри запиту

- Query Param: `API_KEY`
- Query Param: `tenantId`

#### Варіант аутентифікації 3 — OAuth Bearer Token

- Header: `Authorization: Bearer fcat_...`

Додатки, які підключаються через [MCP server](https://docs.fastcomments.com/guide-llm-kit.html), отримують токен через OAuth замість API‑ключа. Цей токен працює на всіх кінцевих точках. Орендатор (tenant) передбачений токеном, тому `tenantId` є необов’язковим, але якщо вказаний, він має відповідати токену. Запити `GET` потребують області `read`, а всі інші методи — області `write`. Відкриття (discovery) починається за адресою `https://fastcomments.com/.well-known/oauth-authorization-server`.

### Читання власних записів

FastComments забезпечує активну-активну доступність. Запити з вашого дата‑центру маршрутизуються до [найближчої точки присутності](https://sophon.fastcomments.com/) від вашої. Це автоматично, і зазвичай ви можете спостерігати семантику «читати‑те‑запис». Якщо ви хочете бути впевненими, що читаєте власні записи, ви можете прив’язати свої запити до певного регіону, використовуючи цей регіон як хост API (проте зазвичай це не потрібно для більшості інтеграцій):

- gdc-oregon.fastcomments.com
- gdc-virginia.fastcomments.com
- gdc-singapore.fastcomments.com
- gdc-falkenstein2.fastcomments.com
- gdc-sao-paulo.fastcomments.com
- eudc-helsinki2.fastcomments.com
- eudc-limburg.fastcomments.com
- eudc-france.fastcomments.com

Зверніть увагу, що якщо ви це робите, можливо, захочете визначити резервний варіант, оскільки ми раніше деактивували вузли входу і використовуємо нові назви для перемикання.

---