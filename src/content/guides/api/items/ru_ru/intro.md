### API FastComments

FastComments предоставляет API для взаимодействия с множеством ресурсов. Создавайте интеграции с нашей платформой или даже собственные клиенты!

В этой документации вы найдете все поддерживаемые API-ресурсы с описанием типов запросов и ответов.

Для корпоративных клиентов весь доступ к API фиксируется в Журнале аудита.

### Сгенерированные SDK

FastComments теперь генерирует [API Spec](https://fastcomments.com/js/swagger.json) из нашего кода (это ещё не завершено, но включает многие API).

У нас также теперь есть SDK для популярных языков:

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

API аутентифицируется путем передачи вашего [api key](https://fastcomments.com/auth/my-account/api-secret) либо в заголовке `X-API-KEY`, либо в параметре запроса `API_KEY`. Для вызовов API вам также понадобится ваш `tenantId`. Его можно получить на той же странице, что и ваш api key.

### Примечание по безопасности

Эти маршруты предназначены для вызова с **серверa**. __DO NOT__ вызывать их из браузера. Делая это, вы выставите ваш API ключ — это даст полный доступ к вашей учетной записи любому, кто может просмотреть исходный код страницы!

#### Вариант аутентификации №1 - Заголовки

- Header: `X-API-KEY`
- Header: `X-TENANT-ID`

#### Вариант аутентификации №2 - Параметры запроса

- Query Param: `API_KEY`
- Query Param: `tenantId`

### Чтение собственных записей

FastComments обеспечивает режим Active-Active. Запросы из вашего дата-центра направляются к [ближайшей точке присутствия](https://sophon.fastcomments.com/) по отношению к вашему. Это происходит автоматически, и обычно вы можете наблюдать семантику «чтение после записи». Если вы хотите быть уверены, что читаете именно свои записи, вы можете привязать запросы к определенному региону, используя этот регион как API-хост (однако это обычно не требуется для большинства интеграций):

- gdc-oregon.fastcomments.com
- gdc-virginia.fastcomments.com
- gdc-singapore.fastcomments.com
- gdc-falkenstein2.fastcomments.com
- gdc-sao-paulo.fastcomments.com
- eudc-helsinki2.fastcomments.com
- eudc-limburg.fastcomments.com
- eudc-france.fastcomments.com

Обратите внимание, что если вы это сделаете, вам может потребоваться определить резервный вариант, поскольку в прошлом мы устаревали некоторые входные узлы и использовали новые имена при переключении.