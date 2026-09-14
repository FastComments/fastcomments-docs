### API FastComments

FastComments предоставляет API для взаимодействия со множеством ресурсов. Создавайте интеграции с нашей платформой или даже разрабатывайте собственных клиентов!

В этой документации вы найдёте все поддерживаемые API‑ресурсы, задокументированные с их типами запросов и ответов.

Для корпоративных клиентов весь доступ к API фиксируется в журнале аудита.

### Сгенерированные SDK

FastComments теперь генерирует [API Spec](https://fastcomments.com/js/swagger.json) из нашего кода (это ещё не полностью завершено, но уже включает многие API).

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

### Аутентификация

API аутентифицируется передачей вашего [api key](https://fastcomments.com/auth/my-account/api-secret) в виде заголовка `X-API-KEY` или параметра запроса `API_KEY`. Вам также понадобится ваш `tenantId` для выполнения вызовов API. Его можно получить на той же странице, где находится ваш api key.

### Замечание по безопасности

Эти маршруты предназначены для вызова с **сервера**. __НЕ ДЕЛАЙТЕ__ вызовов из браузера. Это раскроет ваш API‑ключ, предоставив полный доступ к вашему аккаунту любому, кто сможет просмотреть исходный код страницы!

#### Вариант аутентификации 1 – Заголовки

- Header: `X-API-KEY`
- Header: `X-TENANT-ID`

#### Вариант аутентификации 2 – Параметры запроса

- Query Param: `API_KEY`
- Query Param: `tenantId`

#### Вариант аутентификации 3 – OAuth Bearer Token

- Header: `Authorization: Bearer fcat_...`

Сторонние приложения, такие как Zapier, и клиенты [MCP server](https://docs.fastcomments.com/guide-llm-kit.html) получают токен через OAuth вместо API‑ключа. Этот токен работает со всеми эндпоинтами. tenant подразумевается токеном, поэтому `tenantId` является необязательным, но при указании должен соответствовать токену. Запросы `GET` требуют область `read`, а все остальные методы — область `write`. Полный процесс, включая регистрацию клиента, PKCE, обновление и отзыв токена, описан в разделе [OAuth Authorization](#oauth). Открытие начинается по адресу `https://fastcomments.com/.well-known/oauth-authorization-server`.

### Чтение собственных записей

FastComments обеспечивает активную‑активную доступность. Запросы из вашего дата‑центра направляются к [ближайшей точке присутствия](https://sophon.fastcomments.com/) относительно вас. Это происходит автоматически, и обычно вы наблюдаете семантику «чтение‑после‑записи». Если вам необходимо гарантировать чтение собственных записей, вы можете привязать запросы к определённому региону, используя этот регион в качестве хоста API (обычно это не требуется для большинства интеграций):

- gdc-oregon.fastcomments.com
- gdc-virginia.fastcomments.com
- gdc-singapore.fastcomments.com
- gdc-falkenstein2.fastcomments.com
- gdc-sao-paulo.fastcomments.com
- eudc-helsinki2.fastcomments.com
- eudc-limburg.fastcomments.com
- eudc-france.fastcomments.com

Обратите внимание, что при таком подходе может потребоваться определить резервный вариант, так как в прошлом мы отключали узлы входных точек и вводили новые имена для переключения.