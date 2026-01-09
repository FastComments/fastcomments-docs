### Преглед на API

Collab Chat предоставя три REST API крайни точки за програмиране управлението на текстови разговори. Тези крайни точки ви позволяват да извличате, създавате и изтривате анотации без използване на браузърния widget.

Това са публични крайни точки, които удостоверяват потребителите чрез браузърни бисквитки. Те не използват API ключове. Потребителите трябва да са влезли в FastComments в своя браузър, за да имат достъп до тези крайни точки.

### Базов URL

Всички крайни точки на API за Collab Chat са на:

[inline-code-attrs-start title = 'Базов URL'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/comment-collab-chats
[inline-code-end]

### Удостоверяване

Тези крайни точки удостоверяват потребителите чрез браузърни бисквитки. Те не използват API ключове. Потребителите трябва да са влезли в FastComments в своя браузър, за да имат достъп до тези крайни точки.

### Получаване на всички разговори

Вземете всички текстови разговори за конкретна страница.

#### Крайна точка

[inline-code-attrs-start title = 'GET Крайна точка'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
GET /comment-collab-chats/:tenantId?urlId=<urlId>
[inline-code-end]

#### Параметри

`tenantId` (параметър в пътя, задължителен) е вашият FastComments Tenant ID.

`urlId` (параметър на заявката, задължителен) е идентификаторът на страницата, за която искате да получите разговорите.

#### Отговор

Отговорът включва статуса на API, информация за текущата потребителска сесия, ако е удостоверена, масив от разговори с техните ID-та, URL-и и текстови диапазони, почистен идентификатор на URL, флаг, указващ дали текущият потребител е администратор на сайта или модератор, и подробности за WebSocket връзката за синхронизация в реално време, включително `tenantIdWS`, `urlIdWS` и `userIdWS`.

#### Примерна заявка

[inline-code-attrs-start title = 'Пример за GET заявка'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl "https://fastcomments.com/comment-collab-chats/demo?urlId=my-article-page" \
  --cookie "your-session-cookie"
[inline-code-end]

#### Примерен отговор

[inline-code-attrs-start title = 'Пример за GET отговор'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

### Създаване на разговор

Създайте нов текстов разговор за конкретен избран текст.

#### Крайна точка

[inline-code-attrs-start title = 'POST Крайна точка'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
POST /comment-collab-chats/:tenantId
[inline-code-end]

#### Параметри

`tenantId` (параметър в пътя, задължителен) е вашият FastComments Tenant ID.

Тялото на заявката трябва да е JSON и да включва следните задължителни полета.

`urlId` (string, задължително) е базовият идентификатор на страницата.

`urlIdWithRange` (string, задължително) е URL комбиниран с текстовия диапазон, например `my-page:p:0:15,0:45{abc123}`.

`pageTitle` (string, задължително) е заглавието на страницата.

`selector` (string, задължително) е DOM пътят към елемента, съдържащ избрания текст.

`range` (string, задължително) е сериализираният текстов диапазон във формат `startOffset:endOffset,startOffset:endOffset{checksum}`.

`createdFromCommentId` (string, задължително) е ID-то на коментара, който е инициирал този чат.

`broadcastId` (string, задължително) е UUID за синхронизация в реално време с цел предотвратяване на ехо ефекти.

#### Отговор

Отговорът включва статуса на API и ID-то на новосъздадения разговор.

#### Примерна заявка

[inline-code-attrs-start title = 'Пример за POST заявка'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
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

#### Примерен отговор

[inline-code-attrs-start title = 'Пример за POST отговор'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success",
  "createdChatId": "conv789"
}
[inline-code-end]

### Изтриване на разговор

Изтрийте съществуващ текстов разговор. Тази крайна точка изисква права на администратор или модератор на сайта, или разговорът трябва да е бил създаден от удостоверения потребител.

#### Крайна точка

[inline-code-attrs-start title = 'DELETE Крайна точка'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
DELETE /comment-collab-chats/:tenantId/:chatId
[inline-code-end]

#### Параметри

`tenantId` (параметър в пътя, задължителен) е вашият FastComments Tenant ID.

`chatId` (параметър в пътя, задължителен) е ID-то на разговора, който ще бъде изтрит.

`broadcastId` (тяло на заявката, задължително) е UUID за синхронизация в реално време.

#### Примерна заявка

[inline-code-attrs-start title = 'Пример за DELETE заявка'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl -X DELETE "https://fastcomments.com/comment-collab-chats/demo/conv789" \
  --cookie "your-session-cookie" \
  -H "Content-Type: application/json" \
  -d '{
    "broadcastId": "550e8400-e29b-41d4-a716-446655440001"
  }'
[inline-code-end]

#### Примерен отговор

[inline-code-attrs-start title = 'Пример за DELETE отговор'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success"
}
[inline-code-end]

### Ограничения за честота на заявки

Тези крайни точки подлежат на стандартните ограничения за честота на API на FastComments. Middleware за ограничение на честотата се прилага на ниво наемател (tenant) за предотвратяване на злоупотреби, като същевременно позволява нормални модели на използване.

### Отговори при грешка

Всички крайни точки връщат стандартни HTTP статус кодове. Отговор 400 означава невалидни параметри на заявката. Отговор 401 означава неуспешно удостоверяване. Отговор 403 указва недостатъчни права. Отговор 404 означава, че разговорът не е намерен. Отговор 429 показва превишен лимит на заявките.

Отговорите при грешка включват JSON тяло с подробности:

[inline-code-attrs-start title = 'Пример за отговор при грешка'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "error",
  "message": "Conversation not found"
}
[inline-code-end]

### Проследяване на използването

Създаването на разговори увеличава вашата метрика за използване `conversationCreateCount`. Цялата WebSocket синхронизационна активност увеличава `pubSubMessageCount` и `pubSubBandwidth`. Можете да следите тези метрики в таблото за управление на FastComments под анализа на използването.