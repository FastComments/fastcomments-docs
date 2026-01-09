### Преглед API‑ја

Image Chat пружа три REST API ендпоинта за програмско управљање конверзацијама слика. Ови ендпоинти вам омогућавају да преузимате, креирате и бришете маркере без коришћења видгета у прегледачу.

Сви API ендпоинти захтијевају аутентификацију и прате стандардне FastComments API обрасце. Ово су јавни ендпоинти који аутентификују преко колачића прегледача, а не преко API кључева.

### Base URL

Сви Image Chat API ендпоинти су под:

```
https://fastcomments.com/comment-image-chats
```

### Аутентификација

Ови ендпоинти аутентификују кориснике преко колачића прегледача. Не користе API кључеве. Корисници морају бити пријављени у FastComments у свом прегледачу да би приступили овим ендпоинтима.

### Добијање свих разговора

Преузмите све разговоре везане за одређену слику.

#### Endpoint

```
GET /comment-image-chats/:tenantId?urlId=<urlId>
```

#### Parameters

`tenantId` (path parameter, required) је ваш FastComments Tenant ID.

`urlId` (query parameter, required) је идентификатор слике за који желите да преузмете разговоре.

#### Response

Одговор садржи статус API‑ја, информацију о тренутној корисничкој сесији ако је аутентификована, низ разговора са њиховим ID-јевима, URL-овима и X/Y координатама, очишћени идентификатор URL-а, ознаку да ли је тренутни корисник администратор сајта или модератор, и детаље WebSocket везе за синхронизацију уживо који укључују `tenantIdWS`, `urlIdWS`, и `userIdWS`.

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

### Креирање разговора

Креирајте нови разговор о слици за одређену локацију на слици.

#### Endpoint

```
POST /comment-image-chats/:tenantId
```

#### Parameters

`tenantId` (path parameter, required) је ваш FastComments Tenant ID.

Тело захтјева мора бити у JSON формату и садржати следећа обавезна поља.

`urlId` (string, required) је базни идентификатор странице.

`windowUrlId` (string, required) је URL у комбинацији са извором слике и координатама, на пример `my-page:/images/photo.jpg:25.5:30.2`.

`pageTitle` (string, required) је наслов странице.

`src` (string, required) је URL извор слике.

`x` (number, required) је X координата као проценат (0-100).

`y` (number, required) је Y координата као проценат (0-100).

`createdFromCommentId` (string, required) је ID коментара који је покренуо овај ћаскање.

`broadcastId` (string, required) је UUID за синхронизацију уживо да би се спречили ефекти еха.

#### Response

Одговор садржи статус API‑ја и ID управо креираног разговора.

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

### Брисање разговора

Избришите постојећи разговор о слици. Овај ендпоинт захтијева дозволе администратора или модератора, или разговор мора бити креиран од стране аутентификованог корисника.

#### Endpoint

```
DELETE /comment-image-chats/:tenantId/:chatId
```

#### Parameters

`tenantId` (path parameter, required) је ваш FastComments Tenant ID.

`chatId` (path parameter, required) је ID разговора који се брише.

`broadcastId` (request body, required) је UUID за синхронизацију уживо.

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

### Систем координата

X и Y координате се чувају као проценти димензија слике. X представља хоризонталну позицију од леве ивице (0 = лава ивица, 100 = десна ивица). Y представља вертикалну позицију од врха (0 = врх, 100 = дно).

Ове процентуалне вредности могу садржати децимале за већу прецизност. На пример, x: 25.5 значи 25.5% од леве ивице слике.

### Ограничења учесталости захтева

Ови ендпоинти су подложни стандардним FastComments ограничењима учесталости захтева. Middleware за ограничење примењује се по tenant-у да би се спречила злоупотреба, а истовремено допустило нормално коришћење.

### Одговори са грешком

Сви ендпоинти враћају стандардне HTTP статус кодове. Одговор 400 означава неважеће параметре захтева. Одговор 401 значи да аутентификација није успела. Одговор 403 указује на недовољна овлашћења. Одговор 404 значи да разговор није пронађен. Одговор 429 означава прелазак лимита захтева.

Одговори са грешком укључују JSON тело са детаљима:

```json
{
  "status": "error",
  "message": "Conversation not found"
}
```

### Праћење коришћења

Креирање разговора повећава вашу употребну метрику `conversationCreateCount`. Сва активност WebSocket синхронизације повећава `pubSubMessageCount` и `pubSubBandwidth`. Ове метрике можете пратити у вашем FastComments контролном панелу под аналитиком коришћења.