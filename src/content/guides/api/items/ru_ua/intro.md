### The FastComments API

FastComments предоставляет API для взаимодействия с множеством ресурсов. Создавайте интеграции с нашей платформой или даже пишите собственных клиентов!

В этой документации вы найдёте все поддерживаемые API ресурсы с описанием типов запросов и ответов.

Для корпоративных клиентов весь доступ к API фиксируется в журнале аудита.

### Generated SDKs

FastComments теперь генерирует [Спецификация API](https://fastcomments.com/js/swagger.json) из нашего кода (это ещё не завершено, но включает многие API).

У нас также есть SDK для популярных языков:

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

### Authentication

Доступ к API осуществляется путём передачи вашего [API-ключа](https://fastcomments.com/auth/my-account/api-secret) либо в заголовке `X-API-KEY`, либо в параметре запроса `API_KEY`. Для вызова API вам также понадобится ваш `tenantId`
— его можно получить на той же странице, что и ваш API-ключ.

### Security Note

Эти маршруты предназначены для вызова с **сервера**. __НЕ__ вызывайте их из браузера. В противном случае вы выставите ваш API-ключ — это даст полный доступ к вашей учётной записи
любому, кто может просмотреть исходный код страницы!

#### Authentication Option One - Headers

- Header: `X-API-KEY`
- Header: `X-TENANT-ID`

#### Authentication Option Two - Query Parameters

- Query Param: `API_KEY`
- Query Param: `tenantId`

---