### FastComments API

FastComments надає API для взаємодії з багатьма ресурсами. Створюйте інтеграції з нашою платформою або навіть власні клієнти!

У цій документації ви знайдете всі ресурси, які підтримує API, задокументовані з їхніми типами запитів та відповідей.

Для клієнтів Enterprise увесь доступ до API записується в журнал аудиту.

### Generated SDKs

FastComments тепер генерує [Специфікацію API](https://fastcomments.com/js/swagger.json) з нашого коду (це ще не завершено, але включає багато API).

Ми також тепер маємо SDK для популярних мов:

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

API аутентифікується шляхом передачі вашого [ключа API](https://fastcomments.com/auth/my-account/api-secret) або в заголовку `X-API-KEY`, або в параметрі запиту `API_KEY`. Вам також знадобиться ваш `tenantId` для викликів API. Його можна отримати на тій самій сторінці, що й ваш ключ API.

### Примітка щодо безпеки

Ці маршрути призначені для виклику з **сервера**. __НЕ__ викликайте їх з браузера. Роблячи це, ви зробите ваш ключ API відкритим — це надасть повний доступ до вашого облікового запису будь-кому, хто може переглянути вихідний код сторінки!

#### Варіант автентифікації 1 - Заголовки

- Заголовок: `X-API-KEY`
- Заголовок: `X-TENANT-ID`

#### Варіант автентифікації 2 - Параметри запиту

- Параметр запиту: `API_KEY`
- Параметр запиту: `tenantId`

### Читання власних записів

FastComments забезпечує активно-активну доступність. Запити з вашого датацентру направляються до [найближчого пункту присутності](https://sophon.fastcomments.com/) відносно вашого. Це відбувається автоматично, і зазвичай ви можете спостерігати семантику "читання-після-запису". Якщо ви хочете бути впевненими, що читаєте свої власні записи, ви можете прив'язати свої запити до певного регіону, використавши цей регіон як хост API (однак зазвичай це не потрібно для більшості інтеграцій):

- gdc-oregon.fastcomments.com
- gdc-virginia.fastcomments.com
- gdc-singapore.fastcomments.com
- gdc-falkenstein2.fastcomments.com
- gdc-sao-paulo.fastcomments.com
- eudc-helsinki2.fastcomments.com
- eudc-limburg.fastcomments.com
- eudc-france.fastcomments.com

Зверніть увагу, що якщо ви це зробите, можливо, захочете визначити запасний варіант, оскільки в минулому ми застарілі вузли входу і використовуємо нові імена під час переключення.