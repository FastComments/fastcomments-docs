### API Pregled

Collab Chat пружа три REST API крајње тачке за управљање текстуалним разговорима програмски. Ове крајње тачке вам омогућавају да преузмете, креирате и обришете анотације без употребе видџета у прегледачу.

Ово су јавне крајње тачке које аутентикују кориснике преко колачића прегледача. Оне не користе API кључеве. Корисници морају бити пријављени у FastComments у свом прегледачу да би приступили овим крајњим тачкама.

### Base URL

Све Collab Chat API крајње тачке су под:

[inline-code-attrs-start title = 'Основни URL'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/comment-collab-chats
[inline-code-end]

### Аутентификација

Ове крајње тачке аутентикују кориснике преко колачића прегледача. Оне не користе API кључеве. Корисници морају бити пријављени у FastComments у свом прегледачу да би приступили овим крајњим тачкама.

### Преузми све разговоре

Преузмите све текстуалне разговоре за одређену страницу.

#### Endpoint

[inline-code-attrs-start title = 'GET крајња тачка'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
GET /comment-collab-chats/:tenantId?urlId=<urlId>
[inline-code-end]

#### Параметри

`tenantId` (параметар путање, обавезно) је ваш FastComments Tenant ID.

`urlId` (query параметар, обавезно) је идентификатор странице за коју желите да преузмете разговоре.

#### Одговор

Одговор садржи статус API-ја, информације о тренутној корисничкој сесији ако је аутентификована, низ разговора са њиховим ID-јевима, URL-овима и опсезима текста, очишћени идентификатор URL-а, ознаку која указује да ли је тренутни корисник администратор сајта или модератор, и детаље за WebSocket везу за синхронизацију уживо укључујући `tenantIdWS`, `urlIdWS`, и `userIdWS`.

#### Пример захтева

[inline-code-attrs-start title = 'Пример GET захтева'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl "https://fastcomments.com/comment-collab-chats/demo?urlId=my-article-page" \
  --cookie "your-session-cookie"
[inline-code-end]

#### Пример одговора

[inline-code-attrs-start title = 'Пример GET одговора'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

### Креирај разговор

Креирајте нов текстуални разговор за одређени избор текста.

#### Endpoint

[inline-code-attrs-start title = 'POST крајња тачка'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
POST /comment-collab-chats/:tenantId
[inline-code-end]

#### Параметри

`tenantId` (параметар путање, обавезно) је ваш FastComments Tenant ID.

Тело захтева мора бити у JSON формату и укључивати следећа обавезна поља.

`urlId` (string, обавезно) је базни идентификатор странице.

`urlIdWithRange` (string, обавезно) је URL комбинован са опсегом текста, на пример `my-page:p:0:15,0:45{abc123}`.

`pageTitle` (string, обавезно) је наслов странице.

`selector` (string, обавезно) је DOM пут до елемента који садржи изабрани текст.

`range` (string, обавезно) је сериализовани опсег текста у формату `startOffset:endOffset,startOffset:endOffset{checksum}`.

`createdFromCommentId` (string, обавезно) је ID коментара који је иницирао овај разговор.

`broadcastId` (string, обавезно) је UUID за синхронизацију уживо ради спречавања ефеката еха.

#### Одговор

Одговор садржи статус API-ја и ID новокреираног разговора.

#### Пример захтева

[inline-code-attrs-start title = 'Пример POST захтева'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
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

#### Пример одговора

[inline-code-attrs-start title = 'Пример POST одговора'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success",
  "createdChatId": "conv789"
}
[inline-code-end]

### Избриши разговор

Обришите постојећи текстуални разговор. Ова крајња тачка захтева овлашћења администратора или модератора сајта, или разговор мора бити креирао аутентификовани корисник.

#### Endpoint

[inline-code-attrs-start title = 'DELETE крајња тачка'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
DELETE /comment-collab-chats/:tenantId/:chatId
[inline-code-end]

#### Параметри

`tenantId` (параметар путање, обавезно) је ваш FastComments Tenant ID.

`chatId` (параметар путање, обавезно) је ID разговора који треба обрисати.

`broadcastId` (тело захтева, обавезно) је UUID за синхронизацију уживо.

#### Пример захтева

[inline-code-attrs-start title = 'Пример DELETE захтева'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl -X DELETE "https://fastcomments.com/comment-collab-chats/demo/conv789" \
  --cookie "your-session-cookie" \
  -H "Content-Type: application/json" \
  -d '{
    "broadcastId": "550e8400-e29b-41d4-a716-446655440001"
  }'
[inline-code-end]

#### Пример одговора

[inline-code-attrs-start title = 'Пример DELETE одговора'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success"
}
[inline-code-end]

### Ограничење брзине

Ове крајње тачке су подложне стандардном FastComments ограничавању захтева (rate limiting). Мидлвер за ограничење брзине се примењује по tenant-у како би се спречило злоупотреба, а истовремено омогућили нормални обрасци коришћења.

### Одговори о грешкама

Све крајње тачке враћају стандардне HTTP статус кодове. 400 одговор указује на неважеће параметре захтева. 401 одговор значи да аутентификација није успела. 403 одговор указује на недовољне дозволе. 404 одговор значи да разговор није пронађен. 429 одговор указује да је премашено ограничење брзине.

Одговори о грешкама укључују JSON тело са детаљима:

[inline-code-attrs-start title = 'Пример одговора о грешци'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "error",
  "message": "Conversation not found"
}
[inline-code-end]

### Праћење коришћења

Креирање разговора повећава вашу метрику коришћења `conversationCreateCount`. Сва активност WebSocket синхронизације повећава `pubSubMessageCount` и `pubSubBandwidth`. Можете пратити ове метрике на вашем FastComments контролном панелу у делу за аналитичке податке о коришћењу.