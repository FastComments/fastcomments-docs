### API Pregled

Image Chat пружа три REST API крајње тачке за управљање разговорима о сликама програмски. Ове крајње тачке омогућавају преузимање, креирање и брисање маркера без коришћења widget-а у прегледачу.

Све API крајње тачке захтевају аутентификацију и прате стандардне FastComments API образце. Ово су јавне крајње тачке које аутентификују преко колачића у прегледачу, а не помоћу API кључева.

### Основни URL

Све Image Chat API крајње тачке су под:

```
https://fastcomments.com/comment-image-chats
```

### Аутентификација

Ове крајње тачке аутентификују кориснике преко колачића у прегледачу. Оне не користе API кључеве. Корисници морају бити пријављени у FastComments у свом прегледачу да би приступили овим крајњим тачкама.

### Преглед свих разговора

Преузмите све разговоре о слици за одређену слику.

#### Крајња тачка

```
GET /comment-image-chats/:tenantId?urlId=<urlId>
```

#### Параметри

`tenantId` (path parameter, required) је ваш FastComments Tenant ID.

`urlId` (query parameter, required) је идентификатор слике за коју желите да преузмете разговоре.

#### Одговор

Одговор садржи статус API-ја, информације о тренутној сесији корисника ако је аутентификован, низ разговора са њиховим ИД-овима, URL-овима и X/Y координатама, очишћени идентификатор URL-а, ознаку која указује да ли је тренутни корисник администратор или модератор сајта, и детаље о WebSocket вези за синхронизацију уживо укључујући `tenantIdWS`, `urlIdWS`, и `userIdWS`.

#### Пример захтева

```bash
curl "https://fastcomments.com/comment-image-chats/demo?urlId=my-product-image" \
  --cookie "your-session-cookie"
```

#### Пример одговора

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

#### Крајња тачка

```
POST /comment-image-chats/:tenantId
```

#### Параметри

`tenantId` (path parameter, required) је ваш FastComments Tenant ID.

Тело захтева мора бити у JSON формату и садржати следећа обавезна поља.

`urlId` (string, required) је основни идентификатор странице.

`windowUrlId` (string, required) је URL у комбинaciji са извором слике и координатама, на пример `my-page:/images/photo.jpg:25.5:30.2`.

`pageTitle` (string, required) је наслов странице.

`src` (string, required) је URL извора слике.

`x` (number, required) је X координата као проценат (0-100).

`y` (number, required) је Y координата као проценат (0-100).

`createdFromCommentId` (string, required) је ID коментара који је иницирао овај разговор.

`broadcastId` (string, required) је UUID за синхронизацију уживо како би се спречили ефекти одјечавања (echo).

#### Одговор

Одговор садржи статус API-ја и ID ново креираног разговора.

#### Пример захтева

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

#### Пример одговора

```json
{
  "status": "success",
  "createdChatId": "conv789"
}
```

### Брисање разговора

Избришите постојећи разговор о слици. Ова крајња тачка захтева дозволе администратора или модератора, или разговор мора бити креиран од стране аутентификованог корисника.

#### Крајња тачка

```
DELETE /comment-image-chats/:tenantId/:chatId
```

#### Параметри

`tenantId` (path parameter, required) је ваш FastComments Tenant ID.

`chatId` (path parameter, required) је ID разговора који треба обрисати.

`broadcastId` (request body, required) је UUID за синхронизацију уживо.

#### Пример захтева

```bash
curl -X DELETE "https://fastcomments.com/comment-image-chats/demo/conv789" \
  --cookie "your-session-cookie" \
  -H "Content-Type: application/json" \
  -d '{
    "broadcastId": "550e8400-e29b-41d4-a716-446655440001"
  }'
```

#### Пример одговора

```json
{
  "status": "success"
}
```

### Координатни систем

X и Y координате се чувају као проценти димензија слике. X представља хоризонталну позицију од леве ивице (0 = лева ивица, 100 = десна ивица). Y представља вертикалну позицију од врха (0 = врх, 100 = дно).

Ове процентуалне вредности могу садржати децимале ради прецизности. На пример, x: 25.5 значи 25.5% од леве ивице слике.

### Ограничење броја захтева

Ове крајње тачке подлежу стандардном FastComments ограничењу броја захтева (rate limiting). Middleware за ограничење примењује се по tenant-у да би се спречила злоупотреба док се дозвољава нормалан образац коришћења.

### Одговори са грешком

Све крајње тачке враћају стандардне HTTP статус кодове. Одговор 400 означава неважеће параметре захтева. Одговор 401 значи да аутентификација није успела. Одговор 403 указује на недостатак дозвола. Одговор 404 значи да разговор није пронађен. Одговор 429 означава да је прекорачен лимит захтева.

Одговори са грешком укључују JSON тело са детаљима:

```json
{
  "status": "error",
  "message": "Conversation not found"
}
```

### Праћење употребе

Креирање разговора повећава вашу метрику употребе `conversationCreateCount`. Сва WebSocket sync активност повећава `pubSubMessageCount` и `pubSubBandwidth`. Ове метрике можете пратити на вашем FastComments контролној табли у делу за аналитику употребе.