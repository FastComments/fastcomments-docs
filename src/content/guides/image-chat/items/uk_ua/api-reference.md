### Огляд API

Image Chat надає три REST API кінцеві точки для програмного керування розмовами зображень. Ці кінцеві точки дозволяють отримувати, створювати та видаляти маркери без використання віджета у браузері.

Усі кінцеві точки API вимагають аутентифікацію і дотримуються стандартних шаблонів FastComments API. Це публічні кінцеві точки, які автентифікуються за допомогою cookie браузера, а не API-ключів.

### Базовий URL

Усі кінцеві точки Image Chat API доступні за адресою:

```
https://fastcomments.com/comment-image-chats
```

### Аутентифікація

Ці кінцеві точки автентифікують користувачів за допомогою cookie браузера. Вони не використовують API-ключі. Користувачі повинні бути увійшли в FastComments у своєму браузері, щоб отримати доступ до цих кінцевих точок.

### Отримати всі розмови

Отримати всі розмови зображення для конкретного зображення.

#### Endpoint

```
GET /comment-image-chats/:tenantId?urlId=<urlId>
```

#### Parameters

`tenantId` (path parameter, required) — ваш FastComments Tenant ID.

`urlId` (query parameter, required) — ідентифікатор зображення, для якого потрібно отримати розмови.

#### Response

Відповідь містить статус API, інформацію про поточну сесію користувача, якщо вона аутентифікована, масив розмов з їхніми ID, URL та координатами X/Y, очищений ідентифікатор URL, прапорець, що вказує, чи є поточний користувач адміністратором сайту або модератором, а також деталі WebSocket-з'єднання для синхронізації в реальному часі, включаючи `tenantIdWS`, `urlIdWS` та `userIdWS`.

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

### Створити розмову

Створити нову розмову зображення для конкретного місця на зображенні.

#### Endpoint

```
POST /comment-image-chats/:tenantId
```

#### Parameters

`tenantId` (path parameter, required) — ваш FastComments Tenant ID.

Тіло запиту має бути у форматі JSON і містити наступні обов'язкові поля.

`urlId` (string, required) — базовий ідентифікатор сторінки.

`windowUrlId` (string, required) — URL, що поєднує сторінку з джерелом зображення та координатами, наприклад `my-page:/images/photo.jpg:25.5:30.2`.

`pageTitle` (string, required) — заголовок сторінки.

`src` (string, required) — URL джерела зображення.

`x` (number, required) — координата X у відсотках (0-100).

`y` (number, required) — координата Y у відсотках (0-100).

`createdFromCommentId` (string, required) — ID коментаря, який ініціював цю розмову.

`broadcastId` (string, required) — UUID для синхронізації в реальному часі, щоб уникнути ефекту ехо.

#### Response

Відповідь містить статус API та ID щойно створеної розмови.

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

### Видалити розмову

Видалити існуючу розмову зображення. Ця кінцева точка вимагає прав адміністратора або модератора, або розмова повинна була бути створена автентифікованим користувачем.

#### Endpoint

```
DELETE /comment-image-chats/:tenantId/:chatId
```

#### Parameters

`tenantId` (path parameter, required) — ваш FastComments Tenant ID.

`chatId` (path parameter, required) — ID розмови, яку потрібно видалити.

`broadcastId` (request body, required) — UUID для синхронізації в реальному часі.

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

### Система координат

Координати X та Y зберігаються як відсотки від розмірів зображення. X позначає горизонтальну позицію від лівого краю (0 = лівий край, 100 = правий край). Y позначає вертикальну позицію від верхнього краю (0 = верхній край, 100 = нижній край).

Ці процентні значення можуть містити десяткові дроби для точності. Наприклад, x: 25.5 означає 25.5% від лівого краю зображення.

### Обмеження частоти запитів

Ці кінцеві точки підлягають стандартному лімітуванню FastComments API. Посередник для обмежень застосовується для кожного tenant, щоб запобігти зловживанням, при цьому дозволяючи нормальні сценарії використання.

### Відповіді з помилками

Усі кінцеві точки повертають стандартні HTTP-коди стану. Відповідь 400 вказує на недійсні параметри запиту. Відповідь 401 означає, що аутентифікація не вдалася. Відповідь 403 вказує на недостатні права. Відповідь 404 означає, що розмова не знайдена. Відповідь 429 вказує на перевищення ліміту запитів.

Відповіді з помилками містять JSON-тіло з деталями:

```json
{
  "status": "error",
  "message": "Conversation not found"
}
```

### Відстеження використання

Створення розмов збільшує ваш показник використання `conversationCreateCount`. Уся активність синхронізації через WebSocket збільшує `pubSubMessageCount` та `pubSubBandwidth`. Ви можете відстежувати ці метрики в панелі управління FastComments у розділі аналітики використання.