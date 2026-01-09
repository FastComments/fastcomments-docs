### Обзор API

Collab Chat предоставляет три REST API эндпоинта для программного управления текстовыми разговорами. Эти эндпоинты позволяют получать, создавать и удалять аннотации без использования виджета в браузере.

Это публичные эндпоинты, которые аутентифицируют пользователей через браузерные cookie. Они не используют API-ключи. Пользователи должны быть залогинены в FastComments в своем браузере, чтобы получить доступ к этим эндпоинтам.

### Базовый URL

Все API эндпоинты Collab Chat доступны по адресу:

[inline-code-attrs-start title = 'Базовый URL'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/comment-collab-chats
[inline-code-end]

### Аутентификация

Эти эндпоинты аутентифицируют пользователей через браузерные cookie. Они не используют API-ключи. Пользователи должны быть залогинены в FastComments в своем браузере, чтобы получить доступ к этим эндпоинтам.

### Получить все разговоры

Получите все текстовые разговоры для конкретной страницы.

#### Конечная точка

[inline-code-attrs-start title = 'GET конечная точка'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
GET /comment-collab-chats/:tenantId?urlId=<urlId>
[inline-code-end]

#### Параметры

`tenantId` (параметр пути, обязательный) — это ваш Tenant ID FastComments.

`urlId` (параметр запроса, обязательный) — идентификатор страницы, для которой вы хотите получить разговоры.

#### Ответ

Ответ включает статус API, информацию о текущей сессии пользователя, если он аутентифицирован, массив разговоров с их ID, URL и текстовыми диапазонами, очищенный идентификатор URL, urlIdClean, флаг, указывающий, является ли текущий пользователь администратором сайта или модератором, и данные подключения WebSocket для синхронизации в реальном времени, включая `tenantIdWS`, `urlIdWS` и `userIdWS`.

#### Пример запроса

[inline-code-attrs-start title = 'Пример запроса GET'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl "https://fastcomments.com/comment-collab-chats/demo?urlId=my-article-page" \
  --cookie "your-session-cookie"
[inline-code-end]

#### Пример ответа

[inline-code-attrs-start title = 'Пример ответа GET'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success",
  "user": {
    "id": "user123",
    "username": "john_doe"
  },
  "conversations": [
    {
      "_id": "conv123",
      "urlId": "my-article-page:p:0:15,0:45{abc123}",
      "range": "0:15,0:45{abc123}"
    },
    {
      "_id": "conv456",
      "urlId": "my-article-page:h1:5:20,5:35{def456}",
      "range": "5:20,5:35{def456}"
    }
  ],
  "urlIdClean": "my-article-page",
  "isSiteAdmin": false,
  "tenantIdWS": "demo",
  "urlIdWS": "my-article-page",
  "userIdWS": "user123"
}
[inline-code-end]

### Создать разговор

Создайте новый текстовый разговор для выбранного фрагмента текста.

#### Конечная точка

[inline-code-attrs-start title = 'POST конечная точка'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
POST /comment-collab-chats/:tenantId
[inline-code-end]

#### Параметры

`tenantId` (параметр пути, обязательный) — это ваш Tenant ID FastComments.

Тело запроса должно быть в формате JSON и содержать следующие обязательные поля.

`urlId` (string, required) — базовый идентификатор страницы.

`urlIdWithRange` (string, required) — URL в комбинации с текстовым диапазоном, например `my-page:p:0:15,0:45{abc123}`.

`pageTitle` (string, required) — заголовок страницы.

`selector` (string, required) — DOM-путь к элементу, содержащему выбранный текст.

`range` (string, required) — сериализованный текстовый диапазон в формате `startOffset:endOffset,startOffset:endOffset{checksum}`.

`createdFromCommentId` (string, required) — ID комментария, который инициировал этот чат.

`broadcastId` (string, required) — UUID для синхронизации в реальном времени, чтобы предотвратить эффект эха.

#### Ответ

Ответ включает статус API и ID вновь созданного разговора.

#### Пример запроса

[inline-code-attrs-start title = 'Пример запроса POST'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl -X POST "https://fastcomments.com/comment-collab-chats/demo" \
  --cookie "your-session-cookie" \
  -H "Content-Type: application/json" \
  -d '{
    "urlId": "my-article-page",
    "urlIdWithRange": "my-article-page:p:0:15,0:45{abc123}",
    "pageTitle": "My Article Title",
    "selector": "div#article > p:nth-child(2)",
    "range": "0:15,0:45{abc123}",
    "createdFromCommentId": "comment789",
    "broadcastId": "550e8400-e29b-41d4-a716-446655440000"
  }'
[inline-code-end]

#### Пример ответа

[inline-code-attrs-start title = 'Пример ответа POST'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success",
  "createdChatId": "conv789"
}
[inline-code-end]

### Удалить разговор

Удалите существующий текстовый разговор. Для этого эндпоинта требуются права администратора или модератора сайта, либо разговор должен быть создан аутентифицированным пользователем.

#### Конечная точка

[inline-code-attrs-start title = 'DELETE конечная точка'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
DELETE /comment-collab-chats/:tenantId/:chatId
[inline-code-end]

#### Параметры

`tenantId` (параметр пути, обязательный) — это ваш Tenant ID FastComments.

`chatId` (параметр пути, обязательный) — ID разговора, который нужно удалить.

`broadcastId` (тело запроса, обязательный) — UUID для синхронизации в реальном времени.

#### Пример запроса

[inline-code-attrs-start title = 'Пример запроса DELETE'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl -X DELETE "https://fastcomments.com/comment-collab-chats/demo/conv789" \
  --cookie "your-session-cookie" \
  -H "Content-Type: application/json" \
  -d '{
    "broadcastId": "550e8400-e29b-41d4-a716-446655440001"
  }'
[inline-code-end]

#### Пример ответа

[inline-code-attrs-start title = 'Пример ответа DELETE'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success"
}
[inline-code-end]

### Ограничение запросов

К этим эндпоинтам применяются стандартные ограничения частоты запросов FastComments. Мидлвар ограничения применяется на уровне арендатора, чтобы предотвратить злоупотребления при сохранении нормальных сценариев использования.

### Ответы об ошибках

Все эндпоинты возвращают стандартные HTTP-коды состояния. Ответ 400 означает неверные параметры запроса. Ответ 401 означает, что аутентификация не удалась. Ответ 403 указывает на недостаточные права. Ответ 404 означает, что разговор не найден. Ответ 429 указывает на превышение лимита запросов.

Ответы об ошибках содержат JSON-тело с подробностями:

[inline-code-attrs-start title = 'Пример ответа об ошибке'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "error",
  "message": "Conversation not found"
}
[inline-code-end]

### Отслеживание использования

Создание разговоров увеличивает вашу метрику использования `conversationCreateCount`. Вся активность синхронизации через WebSocket увеличивает `pubSubMessageCount` и `pubSubBandwidth`. Вы можете отслеживать эти метрики на панели FastComments в разделе аналитики использования.

---