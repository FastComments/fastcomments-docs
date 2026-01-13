### Обзор API

Collab Chat предоставляет три REST API конечные точки для управления текстовыми разговорами программно. Эти конечные точки позволяют получать, создавать и удалять аннотации без использования виджета в браузере.

Это публичные конечные точки, которые аутентифицируют пользователей через куки браузера. Они не используют API-ключи. Пользователи должны быть авторизованы в FastComments в своём браузере для доступа к этим конечным точкам.

### Base URL

Все конечные точки Collab Chat API находятся по адресу:

[inline-code-attrs-start title = 'Базовый URL'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/comment-collab-chats
[inline-code-end]

### Аутентификация

Эти конечные точки аутентифицируют пользователей через куки браузера. Они не используют API-ключи. Пользователи должны быть авторизованы в FastComments в своём браузере для доступа к этим конечным точкам.

### Получить все разговоры

Получите все текстовые разговоры для конкретной страницы.

#### Endpoint

[inline-code-attrs-start title = 'GET-эндпоинт'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
GET /comment-collab-chats/:tenantId?urlId=<urlId>
[inline-code-end]

#### Параметры

`tenantId` (параметр пути, обязательный) — ваш FastComments Tenant ID.

`urlId` (параметр запроса, обязательный) — идентификатор страницы, для которой вы хотите получить разговоры.

#### Ответ

В ответе возвращаются статус API, информация о текущей сессии пользователя, если он аутентифицирован, массив разговоров с их ID, URL и диапазонами текста, очищенный идентификатор URL, флаг, указывающий, является ли текущий пользователь администратором сайта или модератором, и данные для подключения по WebSocket для синхронизации в реальном времени, включая `tenantIdWS`, `urlIdWS` и `userIdWS`.

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

#### Endpoint

[inline-code-attrs-start title = 'POST-эндпоинт'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
POST /comment-collab-chats/:tenantId
[inline-code-end]

#### Параметры

`tenantId` (параметр пути, обязательный) — ваш FastComments Tenant ID.

Тело запроса должно быть в формате JSON и содержать следующие обязательные поля.

`urlId` (string, обязательный) — базовый идентификатор страницы.

`urlIdWithRange` (string, обязательный) — URL в сочетании с диапазоном текста, например `my-page:p:0:15,0:45{abc123}`.

`pageTitle` (string, обязательный) — заголовок страницы.

`selector` (string, обязательный) — путь в DOM к элементу, содержащему выделенный текст.

`range` (string, обязательный) — сериализованный диапазон текста в формате `startOffset:endOffset,startOffset:endOffset{checksum}`.

`createdFromCommentId` (string, обязательный) — ID комментария, который инициировал этот чат.

`broadcastId` (string, обязательный) — UUID для синхронизации в реальном времени, чтобы предотвратить эффект эха.

#### Ответ

В ответе возвращаются статус API и ID вновь созданного разговора.

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

Удалите существующий текстовый разговор. Для этого эндпоинта требуются права администратора или модератора сайта, либо разговор должен был быть создан аутентифицированным пользователем.

#### Endpoint

[inline-code-attrs-start title = 'DELETE-эндпоинт'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
DELETE /comment-collab-chats/:tenantId/:chatId
[inline-code-end]

#### Параметры

`tenantId` (параметр пути, обязательный) — ваш FastComments Tenant ID.

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

### Ограничение скорости (Rate Limiting)

Эти конечные точки подпадают под стандартные ограничения скорости FastComments API. Middleware ограничения скорости применяется по каждому tenant, чтобы предотвратить злоупотребления и при этом позволять нормальные сценарии использования.

### Ответы с ошибками

Все конечные точки возвращают стандартные коды состояния HTTP. Ответ 400 указывает на неверные параметры запроса. Ответ 401 означает ошибку аутентификации. Ответ 403 указывает на недостаточные права. Ответ 404 означает, что разговор не найден. Ответ 429 указывает на превышение лимита запросов.

Ответы с ошибками содержат JSON-тело с подробностями:

[inline-code-attrs-start title = 'Пример ответа с ошибкой'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "error",
  "message": "Conversation not found"
}
[inline-code-end]

### Отслеживание использования

Создание разговоров увеличивает вашу метрику использования `conversationCreateCount`. Вся активность синхронизации WebSocket увеличивает `pubSubMessageCount` и `pubSubBandwidth`. Вы можете отслеживать эти метрики в панели управления FastComments в разделе аналитики использования.