### API FastComments

FastComments предоставляет API для взаимодействия со многими ресурсами. Создавайте интеграции с нашей платформой или даже собственные клиенты!

В этой документации вы найдёте все ресурсы, поддерживаемые API, задокументированные с их типами запросов и ответов.

Для корпоративных клиентов весь доступ к API фиксируется в журнале аудита.

### Сгенерированные SDK

FastComments теперь генерирует [API Spec](https://fastcomments.com/js/swagger.json) из нашего кода (это ещё не завершено, но включает многие API).

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

API аутентифицируется путем передачи вашего [ключа API](https://fastcomments.com/auth/my-account/api-secret) либо в заголовке `X-API-KEY`, либо в параметре запроса `API_KEY`. Вам также понадобится ваш `tenantId`
для вызовов API. Он может быть получен на той же странице, что и ваш api key.

### Примечание по безопасности

Эти маршруты предназначены для вызова с **сервера**. __НЕ ВЫЗЫВАЙТЕ__ их из браузера. Это приведёт к раскрытию вашего ключа API — любой, кто сможет просмотреть исходный код страницы, получит полный доступ к вашему аккаунту!

#### Вариант аутентификации №1 — заголовки

- Заголовок: `X-API-KEY`
- Заголовок: `X-TENANT-ID`

#### Вариант аутентификации №2 — параметры запроса

- Параметр запроса: `API_KEY`
- Параметр запроса: `tenantId`

### Чтение собственных записей

FastComments обеспечивает режим Active-Active. Запросы с вашего дата-центра направляются к [ближайшей точке присутствия](https://sophon.fastcomments.com/) к вам. Это происходит автоматически, и обычно вы можете наблюдать семантику «read-your-write». Если вы хотите гарантированно читать свои собственные записи, вы можете привязать ваши запросы к определённому региону, используя этот регион в качестве хоста API (однако для большинства интеграций это обычно не требуется):

- gdc-oregon.fastcomments.com
- gdc-virginia.fastcomments.com
- gdc-singapore.fastcomments.com
- gdc-falkenstein2.fastcomments.com
- gdc-sao-paulo.fastcomments.com
- eudc-helsinki2.fastcomments.com
- eudc-limburg.fastcomments.com
- eudc-france.fastcomments.com

Обратите внимание, что если вы это сделаете, возможно, стоит задать запасной вариант, поскольку в прошлом мы помечали узлы входа устаревшими и использовали новые имена при переключении.