### FastComments API

FastComments предоставляет API для взаимодействия со множеством ресурсов. Создавайте интеграции с нашей платформой или даже разрабатывайте собственные клиенты!

В этой документации вы найдёте все ресурсы, поддерживаемые API, задокументированные с их типами запросов и ответов.

Для корпоративных клиентов весь доступ к API фиксируется в журнале аудита.

### Сгенерированные SDK

FastComments теперь генерирует [Спецификация API](https://fastcomments.com/js/swagger.json) из нашего кода (это пока не завершено, но включает множество API).

Также у нас теперь есть SDK для популярных языков:

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

### Аутентификация

API аутентифицируется путём передачи вашего [ключа API](https://fastcomments.com/auth/my-account/api-secret) либо в заголовке `X-API-KEY`, либо в параметре запроса `API_KEY`. Вам также понадобится ваш `tenantId` для выполнения вызовов API. Его можно получить на той же странице, что и ваш ключ API.

### Примечание по безопасности

Эти маршруты предназначены для вызова с **сервера**. __НЕ__ вызывайте их из браузера. В противном случае ваш ключ API будет раскрыт — это предоставит полный доступ к вашему аккаунту любому, кто сможет просмотреть исходный код страницы!

#### Вариант аутентификации №1 - Заголовки

- Заголовок: `X-API-KEY`
- Заголовок: `X-TENANT-ID`

#### Вариант аутентификации №2 - Параметры запроса

- Параметр запроса: `API_KEY`
- Параметр запроса: `tenantId`

---