### API FastComments

FastComments предоставляет API для взаимодействия со множеством ресурсов. Создавайте интеграции с нашей платформой или даже создавайте собственные клиенты!

В этой документации вы найдёте все поддерживаемые API ресурсы, задокументированные с их типами запросов и ответов.

Для корпоративных клиентов весь доступ к API фиксируется в журнале аудита.

### Сгенерированные SDK

FastComments теперь генерирует [спецификацию API](https://fastcomments.com/js/swagger.json) из нашего кода (это ещё не полностью завершено, но включает многие API).

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

API аутентифицируется передачей вашего [api‑ключа](https://fastcomments.com/auth/my-account/api-secret) в виде заголовка `X-API-KEY` или параметра запроса `API_KEY`. Вам также понадобится ваш `tenantId` для выполнения API‑вызовов. Его можно получить на той же странице, где находится ваш api‑ключ.

### Примечание по безопасности

Эти маршруты предназначены для вызова с **сервера**. __НЕ ДЕЛАЙТЕ__ вызовов из браузера. Это раскроет ваш API‑ключ — предоставит полный доступ к вашему аккаунту любому, кто сможет просмотреть исходный код страницы!

#### Вариант аутентификации 1 — Заголовки

- Заголовок: `X-API-KEY`
- Заголовок: `X-TENANT-ID`

#### Вариант аутентификации 2 — Параметры запроса

- Параметр запроса: `API_KEY`
- Параметр запроса: `tenantId`

#### Вариант аутентификации 3 — OAuth Bearer Token

- Заголовок: `Authorization: Bearer fcat_...`

Приложения, подключающиеся через [MCP‑сервер](https://docs.fastcomments.com/guide-llm-kit.html), получают токен через OAuth вместо API‑ключа. Этот токен работает на всех конечных точках. Тенант подразумевается токеном, поэтому `tenantId` необязателен, но если указан, он должен соответствовать токену. Запросы `GET` требуют область `read`, а все остальные методы — область `write`. Открытие начинается по адресу `https://fastcomments.com/.well-known/oauth-authorization-server`.

### Чтение собственных записей

FastComments обеспечивает активную‑активную доступность. Запросы из вашего дата‑центра направляются к [ближайшей точке присутствия](https://sophon.fastcomments.com/) относительно вас. Это происходит автоматически, и обычно вы можете наблюдать семантику «чтение‑после‑записи». Если вы хотите гарантировать чтение собственных записей, можете привязать запросы к определённому региону, используя этот регион в качестве хоста API (однако обычно это не требуется для большинства интеграций):

- gdc-oregon.fastcomments.com
- gdc-virginia.fastcomments.com
- gdc-singapore.fastcomments.com
- gdc-falkenstein2.fastcomments.com
- gdc-sao-paulo.fastcomments.com
- eudc-helsinki2.fastcomments.com
- eudc-limburg.fastcomments.com
- eudc-france.fastcomments.com

Обратите внимание, что если вы сделаете это, вам может потребоваться определить резервный вариант, так как мы в прошлом отключали узлы входных точек и используем новые имена для переключения.

---