### API Overview

Image Chat предоставляет три REST API-эндпоинта для программного управления беседами по изображению. Эти эндпоинты позволяют получать, создавать и удалять маркеры без использования виджета в браузере.

Все API-эндпоинты требуют аутентификации и следуют стандартным паттернам FastComments API. Это публичные эндпоинты, которые аутентифицируются через cookie браузера, а не с помощью API-ключей.

### Base URL

Все API-эндпоинты Image Chat находятся по адресу:

```
https://fastcomments.com/comment-image-chats
```

### Authentication

Эти эндпоинты аутентифицируют пользователей через cookie браузера. Они не используют API-ключи. Пользователь должен быть залогинен в FastComments в своём браузере, чтобы получить доступ к этим эндпоинтам.

### Get All Conversations

Получить все беседы по изображению для конкретного изображения.

#### Endpoint

```
GET /comment-image-chats/:tenantId?urlId=<urlId>
```

#### Parameters

`tenantId` (параметр пути, обязателен) — ваш FastComments Tenant ID.

`urlId` (параметр запроса, обязателен) — идентификатор изображения, для которого нужно получить беседы.

#### Response

В ответе содержится статус API, информация о текущей сессии пользователя, если пользователь аутентифицирован, массив бесед с их ID, URL и координатами X/Y, очищенный идентификатор URL, флаг, указывающий, является ли текущий пользователь администратором сайта или модератором, и детали подключения WebSocket для синхронизации в реальном времени, включая `tenantIdWS`, `urlIdWS`, и `userIdWS`.

#### Example Request

```bash
curl "https://fastcomments.com/comment-image-chats/demo?urlId=my-product-image" \
  --cookie "your-session-cookie"
```

#### Example Response

```json
{
  "status": "success",
  "user": {
    "id": "user123",
    "username": "john_doe"
  },
  "conversations": [
    {
      "_id": "conv123",
      "urlId": "my-product-image:/images/product.jpg:25:30",
      "x": 25.5,
      "y": 30.2
    },
    {
      "_id": "conv456",
      "urlId": "my-product-image:/images/product.jpg:60:45",
      "x": 60.8,
      "y": 45.1
    }
  ],
  "urlIdClean": "my-product-image",
  "isSiteAdmin": false,
  "tenantIdWS": "demo",
  "urlIdWS": "my-product-image",
  "userIdWS": "user123"
}
```

### Create Conversation

Создать новую беседу по изображению для конкретного места на изображении.

#### Endpoint

```
POST /comment-image-chats/:tenantId
```

#### Parameters

`tenantId` (параметр пути, обязателен) — ваш FastComments Tenant ID.

Тело запроса должно быть в формате JSON и содержать следующие обязательные поля.

`urlId` (string, обязательное) — базовый идентификатор страницы.

`windowUrlId` (string, обязательное) — URL, объединённый с источником изображения и координатами, например `my-page:/images/photo.jpg:25.5:30.2`.

`pageTitle` (string, обязательное) — заголовок страницы.

`src` (string, обязательное) — URL-источник изображения.

`x` (number, обязательное) — координата X в процентах (0-100).

`y` (number, обязательное) — координата Y в процентах (0-100).

`createdFromCommentId` (string, обязательное) — ID комментария, который инициировал этот чат.

`broadcastId` (string, обязательное) — UUID для синхронизации в реальном времени, чтобы предотвратить эффект эха.

#### Response

В ответе содержится статус API и ID вновь созданной беседы.

#### Example Request

```bash
curl -X POST "https://fastcomments.com/comment-image-chats/demo" \
  --cookie "your-session-cookie" \
  -H "Content-Type: application/json" \
  -d '{
    "urlId": "my-product-image",
    "windowUrlId": "my-product-image:/images/product.jpg:25.5:30.2",
    "pageTitle": "Product Gallery",
    "src": "/images/product.jpg",
    "x": 25.5,
    "y": 30.2,
    "createdFromCommentId": "comment789",
    "broadcastId": "550e8400-e29b-41d4-a716-446655440000"
  }'
```

#### Example Response

```json
{
  "status": "success",
  "createdChatId": "conv789"
}
```

### Delete Conversation

Удалить существующую беседу по изображению. Для этого эндпоинта требуются права администратора или модератора сайта, либо беседа должна быть создана аутентифицированным пользователем.

#### Endpoint

```
DELETE /comment-image-chats/:tenantId/:chatId
```

#### Parameters

`tenantId` (параметр пути, обязателен) — ваш FastComments Tenant ID.

`chatId` (параметр пути, обязателен) — ID беседы, которую нужно удалить.

`broadcastId` (тело запроса, обязательно) — UUID для синхронизации в реальном времени.

#### Example Request

```bash
curl -X DELETE "https://fastcomments.com/comment-image-chats/demo/conv789" \
  --cookie "your-session-cookie" \
  -H "Content-Type: application/json" \
  -d '{
    "broadcastId": "550e8400-e29b-41d4-a716-446655440001"
  }'
```

#### Example Response

```json
{
  "status": "success"
}
```

### Coordinate System

Координаты X и Y хранятся в процентах от размеров изображения. X представляет горизонтальную позицию от левого края (0 = левый край, 100 = правый край). Y представляет вертикальную позицию от верхнего края (0 = верхний край, 100 = нижний край).

Эти процентные значения могут содержать десятичные дроби для точности. Например, x: 25.5 означает 25.5% от левого края изображения.

### Rate Limiting

Эти эндпоинты подчиняются стандартному ограничению частоты запросов FastComments. Middleware ограничения частоты применяется на уровне tenant для предотвращения злоупотреблений при сохранении нормальных сценариев использования.

### Error Responses

Все эндпоинты возвращают стандартные HTTP-коды состояния. Код 400 указывает на неверные параметры запроса. Код 401 означает, что аутентификация не удалась. Код 403 указывает на недостаточные права. Код 404 означает, что беседа не найдена. Код 429 указывает на превышение лимита запросов.

Ответы об ошибках включают JSON-тело с деталями:

```json
{
  "status": "error",
  "message": "Conversation not found"
}
```

### Usage Tracking

Создание бесед увеличивает вашу метрику использования `conversationCreateCount`. Вся активность синхронизации через WebSocket увеличивает `pubSubMessageCount` и `pubSubBandwidth`. Вы можете отслеживать эти метрики на панели управления FastComments в разделе аналитики использования.

---