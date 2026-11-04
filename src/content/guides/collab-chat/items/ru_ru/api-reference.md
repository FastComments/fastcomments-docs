### Обзор API

Collab Chat предоставляет три REST API-эндпоинта для программного управления текстовыми беседами. Эти эндпоинты позволяют получать, создавать и удалять аннотации без использования браузерного виджета.

Это публичные эндпоинты, которые аутентифицируют пользователей через cookies браузера. Они не используют API-ключи. Пользователи должны быть авторизованы в FastComments в своём браузере для доступа к этим эндпоинтам.

### Базовый URL

Все эндпоинты API Collab Chat находятся по адресу:

[inline-code-attrs-start title = 'Базовый URL'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/comment-collab-chats
[inline-code-end]

### Аутентификация

Эти эндпоинты аутентифицируют пользователей через cookies браузера. Они не используют API-ключи. Пользователи должны быть авторизованы в FastComments в своём браузере для доступа к этим эндпоинтам.

### Получение всех бесед

Получить все текстовые беседы для конкретной страницы.

#### Эндпоинт

[inline-code-attrs-start title = 'GET Эндпоинт'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
GET /comment-collab-chats/:tenantId?urlId=<urlId>
[inline-code-end]

#### Параметры

`tenantId` (параметр пути, обязательный) — это ваш FastComments Tenant ID.

`urlId` (параметр запроса, обязательный) — это идентификатор страницы, для которой вы хотите получить беседы.

#### Ответ

Ответ включает статус API, информацию о текущей сессии пользователя, если он аутентифицирован, массив бесед с их ID, URL и текстовыми диапазонами, очищенный идентификатор URL, флаг, указывающий, является ли текущий пользователь администратором сайта или модератором, а также детали WebSocket-соединения для синхронизации в реальном времени, включая `tenantIdWS`, `urlIdWS` и `userIdWS`.

#### Пример запроса

[inline-code-attrs-start title = 'Пример GET-запроса'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl "https://fastcomments.com/comment-collab-chats/demo?urlId=my-article-page" \
  --cookie "your-session-cookie"
[inline-code-end]

#### Пример ответа

[inline-code-attrs-start title = 'Пример GET-ответа'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

### Создание беседы

Создать новую текстовую беседу для конкретного выделенного текста.

#### Эндпоинт

[inline-code-attrs-start title = 'POST Эндпоинт'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
POST /comment-collab-chats/:tenantId
[inline-code-end]

#### Параметры

`tenantId` (параметр пути, обязательный) — это ваш FastComments Tenant ID.

Тело запроса должно быть в формате JSON и включать следующие обязательные поля.

`urlId` (строка, обязательный) — это базовый идентификатор страницы.

`urlIdWithRange` (строка, обязательный) — это URL, объединённый с текстовым диапазоном, например `my-page:p:0:15,0:45{abc123}`.

`pageTitle` (строка, обязательный) — это заголовок страницы.

`selector` (строка, обязательный) — это DOM-путь к элементу, содержащему выделенный текст.

`range` (строка, обязательный) — это сериализованный текстовый диапазон в формате `startOffset:endOffset,startOffset:endOffset{checksum}`.

`createdFromCommentId` (строка, обязательный) — это ID комментария, который инициировал эту беседу.

`broadcastId` (строка, обязательный) — это UUID для синхронизации в реальном времени, предотвращающий эффект эха.

#### Ответ

Ответ включает статус API и ID новой созданной беседы.

#### Пример запроса

[inline-code-attrs-start title = 'Пример POST-запроса'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Пример POST-ответа'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success",
  "createdChatId": "conv789"
}
[inline-code-end]

### Удаление беседы

Удалить существующую текстовую беседу. Этот эндпоинт требует прав администратора или модератора, либо беседа должна быть создана аутентифицированным пользователем.

#### Эндпоинт

[inline-code-attrs-start title = 'DELETE Эндпоинт'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
DELETE /comment-collab-chats/:tenantId/:chatId
[inline-code-end]

#### Параметры

`tenantId` (параметр пути, обязательный) — это ваш FastComments Tenant ID.

`chatId` (параметр пути, обязательный) — это ID беседы, которую нужно удалить.

`broadcastId` (тело запроса, обязательный) — это UUID для синхронизации в реальном времени.

#### Пример запроса

[inline-code-attrs-start title = 'Пример DELETE-запроса'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl -X DELETE "https://fastcomments.com/comment-collab-chats/demo/conv789" \
  --cookie "your-session-cookie" \
  -H "Content-Type: application/json" \
  -d '{
    "broadcastId": "550e8400-e29b-41d4-a716-446655440001"
  }'
[inline-code-end]

#### Пример ответа

[inline-code-attrs-start title = 'Пример DELETE-ответа'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success"
}
[inline-code-end]

### Ограничение частоты запросов

Эти эндпоинты подчиняются стандартным ограничениям частоты запросов FastComments API. Middleware ограничения частоты применяется для каждого тенанта, чтобы предотвратить злоупотребления, при этом допуская нормальные сценарии использования.

### Ответы с ошибками

Все эндпоинты возвращают стандартные HTTP-коды состояния. Ответ 400 указывает на недопустимые параметры запроса. Ответ 401 означает, что аутентификация не пройдена. Ответ 403 указывает на недостаточные права. Ответ 404 означает, что беседа не найдена. Ответ 429 указывает на превышение лимита частоты запросов.

Ответы с ошибками включают JSON-тело с деталями:

[inline-code-attrs-start title = 'Пример ответа с ошибкой'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "error",
  "message": "Conversation not found"
}
[inline-code-end]

### Учёт использования

Создание бесед увеличивает вашу метрику использования `conversationCreateCount`. Вся активность синхронизации через WebSocket увеличивает `pubSubMessageCount` и `pubSubBandwidth`. Вы можете отслеживать эти метрики в панели управления FastComments в разделе аналитики использования.