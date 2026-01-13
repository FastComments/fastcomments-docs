### API Overview

Image Chat предоставляет три REST API-эндпойнта для программного управления обсуждениями изображений. Эти эндпойнты позволяют получать, создавать и удалять маркеры без использования браузерного виджета.

Все API-эндпойнты требуют аутентификации и следуют стандартным шаблонам FastComments API. Это публичные эндпойнты, которые аутентифицируются через cookie браузера, а не через API-ключи.

### Base URL

Все эндпойнты Image Chat API находятся по адресу:

```
https://fastcomments.com/comment-image-chats
```

### Authentication

Эти эндпойнты аутентифицируют пользователей через cookie браузера. Они не используют API-ключи. Пользователь должен быть вошедшим в FastComments в своем браузере, чтобы получить доступ к этим эндпойнтам.

### Get All Conversations

Получить все обсуждения изображений для конкретного изображения.

#### Endpoint

```
GET /comment-image-chats/:tenantId?urlId=<urlId>
```

#### Parameters

`tenantId` (параметр пути, обязательный) — ваш FastComments Tenant ID.

`urlId` (параметр запроса, обязательный) — идентификатор изображения, для которого вы хотите получить обсуждения.

#### Response

В ответ включены статус API, информация о текущей сессии пользователя, если он аутентифицирован, массив обсуждений с их идентификаторами, URL и координатами X/Y, очищенный идентификатор URL, флаг, указывающий, является ли текущий пользователь администратором сайта или модератором, и детали подключения WebSocket для синхронизации в реальном времени, включая `tenantIdWS`, `urlIdWS` и `userIdWS`.

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

Создать новое обсуждение изображения для конкретного места на изображении.

#### Endpoint

```
POST /comment-image-chats/:tenantId
```

#### Parameters

`tenantId` (параметр пути, обязательный) — ваш FastComments Tenant ID.

Тело запроса должно быть в формате JSON и содержать следующие обязательные поля.

`urlId` (string, обязательный) — базовый идентификатор страницы.

`windowUrlId` (string, обязательный) — URL, объединённый с источником изображения и координатами, например `my-page:/images/photo.jpg:25.5:30.2`.

`pageTitle` (string, обязательный) — заголовок страницы.

`src` (string, обязательный) — URL источника изображения.

`x` (number, обязательный) — координата X в процентах (0-100).

`y` (number, обязательный) — координата Y в процентах (0-100).

`createdFromCommentId` (string, обязательный) — ID комментария, который инициировал этот чат.

`broadcastId` (string, обязательный) — UUID для синхронизации в реальном времени, чтобы предотвратить эффект эхо.

#### Response

В ответ включены статус API и ID вновь созданного обсуждения.

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

Удалить существующее обсуждение изображения. Для этого эндпойнта требуются права администратора или модератора, либо обсуждение должно быть создано аутентифицированным пользователем.

#### Endpoint

```
DELETE /comment-image-chats/:tenantId/:chatId
```

#### Parameters

`tenantId` (параметр пути, обязательный) — ваш FastComments Tenant ID.

`chatId` (параметр пути, обязательный) — ID обсуждения, которое нужно удалить.

`broadcastId` (тело запроса, обязательный) — UUID для синхронизации в реальном времени.

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

Координаты X и Y хранятся в виде процентов от размеров изображения. X представляет горизонтальное положение от левого края (0 = левый край, 100 = правый край). Y представляет вертикальное положение от верхнего края (0 = верхний край, 100 = нижний край).

Эти процентные значения могут содержать десятичные дроби для точности. Например, x: 25.5 означает 25.5% от левого края изображения.

### Rate Limiting

Этим эндпойнтам применяются стандартные ограничения по частоте запросов FastComments. Middleware ограничения по частоте применяется на уровне каждого tenant, чтобы предотвратить злоупотребления и одновременно позволить нормальные модели использования.

### Error Responses

Все эндпойнты возвращают стандартные HTTP-коды состояния. Ответ 400 указывает на неверные параметры запроса. Ответ 401 означает ошибку аутентификации. Ответ 403 указывает на недостаточные права. Ответ 404 означает, что обсуждение не найдено. Ответ 429 указывает на превышение лимита запросов.

Ответы об ошибках включают JSON-тело с деталями:

```json
{
  "status": "error",
  "message": "Conversation not found"
}
```

### Usage Tracking

Создание обсуждений увеличивает ваш метрик использования `conversationCreateCount`. Вся активность синхронизации через WebSocket увеличивает `pubSubMessageCount` и `pubSubBandwidth`. Вы можете отслеживать эти метрики на панели FastComments в разделе аналитики использования.