### Огляд API

Collab Chat надає три REST API кінцеві точки для програмного керування текстовими розмовами. Ці кінцеві точки дозволяють отримувати, створювати та видаляти анотації без використання віджета в браузері.

Це публічні кінцеві точки, які автентифікують користувачів через cookie браузера. Вони не використовують API ключі. Користувачі повинні бути увійшли до FastComments у своєму браузері, щоб мати доступ до цих кінцевих точок.

### Базовий URL

Усі кінцеві точки API Collab Chat знаходяться за адресою:

[inline-code-attrs-start title = 'Базовий URL'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/comment-collab-chats
[inline-code-end]

### Аутентифікація

Ці кінцеві точки автентифікують користувачів через cookie браузера. Вони не використовують API ключі. Користувачі повинні бути увійшли до FastComments у своєму браузері, щоб мати доступ до цих кінцевих точок.

### Отримати всі розмови

Отримати всі текстові розмови для конкретної сторінки.

#### Кінцева точка

[inline-code-attrs-start title = 'GET кінцева точка'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
GET /comment-collab-chats/:tenantId?urlId=<urlId>
[inline-code-end]

#### Параметри

`tenantId` (path parameter, required) — це ваш FastComments Tenant ID.

`urlId` (query parameter, required) — це ідентифікатор сторінки, для якої потрібно отримати розмови.

#### Відповідь

У відповіді міститься статус API, інформація про поточну сесію користувача, якщо автентифіковано, масив розмов з їх ідентифікаторами, URL та текстовими діапазонами, очищений ідентифікатор URL, прапорець, що вказує, чи є поточний користувач адміністратором сайту або модератором, та деталі підключення WebSocket для синхронізації в реальному часі, включаючи `tenantIdWS`, `urlIdWS` та `userIdWS`.

#### Приклад запиту

[inline-code-attrs-start title = 'Приклад GET-запиту'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl "https://fastcomments.com/comment-collab-chats/demo?urlId=my-article-page" \
  --cookie "your-session-cookie"
[inline-code-end]

#### Приклад відповіді

[inline-code-attrs-start title = 'Приклад відповіді GET'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

### Створити розмову

Створити нову текстову розмову для конкретного виділення тексту.

#### Кінцева точка

[inline-code-attrs-start title = 'POST кінцева точка'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
POST /comment-collab-chats/:tenantId
[inline-code-end]

#### Параметри

`tenantId` (path parameter, required) — це ваш FastComments Tenant ID.

Тіло запиту має бути в JSON і містити ці обов'язкові поля.

`urlId` (string, required) — це базовий ідентифікатор сторінки.

`urlIdWithRange` (string, required) — це URL у поєднанні з текстовим діапазоном, наприклад `my-page:p:0:15,0:45{abc123}`.

`pageTitle` (string, required) — це заголовок сторінки.

`selector` (string, required) — це DOM-шлях до елемента, що містить виділений текст.

`range` (string, required) — це серіалізований текстовий діапазон у форматі `startOffset:endOffset,startOffset:endOffset{checksum}`.

`createdFromCommentId` (string, required) — це ідентифікатор коментаря, який ініціював цю розмову.

`broadcastId` (string, required) — це UUID для синхронізації в реальному часі, щоб запобігти ефектам ехо.

#### Відповідь

У відповіді міститься статус API та ідентифікатор щойно створеної розмови.

#### Приклад запиту

[inline-code-attrs-start title = 'Приклад POST-запиту'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
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

#### Приклад відповіді

[inline-code-attrs-start title = 'Приклад відповіді POST'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success",
  "createdChatId": "conv789"
}
[inline-code-end]

### Видалити розмову

Видалити існуючу текстову розмову. Ця кінцева точка вимагає прав адміністратора або модератора, або розмова має бути створена автентифікованим користувачем.

#### Кінцева точка

[inline-code-attrs-start title = 'DELETE кінцева точка'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
DELETE /comment-collab-chats/:tenantId/:chatId
[inline-code-end]

#### Параметри

`tenantId` (path parameter, required) — це ваш FastComments Tenant ID.

`chatId` (path parameter, required) — це ідентифікатор розмови, яку потрібно видалити.

`broadcastId` (request body, required) — це UUID для синхронізації в реальному часі.

#### Приклад запиту

[inline-code-attrs-start title = 'Приклад DELETE-запиту'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl -X DELETE "https://fastcomments.com/comment-collab-chats/demo/conv789" \
  --cookie "your-session-cookie" \
  -H "Content-Type: application/json" \
  -d '{
    "broadcastId": "550e8400-e29b-41d4-a716-446655440001"
  }'
[inline-code-end]

#### Приклад відповіді

[inline-code-attrs-start title = 'Приклад відповіді DELETE'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success"
}
[inline-code-end]

### Обмеження швидкості

Ці кінцеві точки підпадають під стандартне обмеження швидкості API FastComments. Middleware для обмеження частоти застосовується на рівні tenant, щоб запобігти зловживанню, одночасно дозволяючи нормальні сценарії використання.

### Відповіді з помилками

Усі кінцеві точки повертають стандартні HTTP коди стану. Відповідь 400 означає невірні параметри запиту. Відповідь 401 означає, що автентифікація не вдалася. Відповідь 403 вказує на недостатні дозволи. Відповідь 404 означає, що розмова не знайдена. Відповідь 429 означає перевищення ліміту запитів.

Відповіді з помилками містять тіло JSON з деталями:

[inline-code-attrs-start title = 'Приклад відповіді з помилкою'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "error",
  "message": "Conversation not found"
}
[inline-code-end]

### Відстеження використання

Створення розмов збільшує ваш рахунок використання `conversationCreateCount`. Уся активність синхронізації через WebSocket збільшує `pubSubMessageCount` та `pubSubBandwidth`. Ви можете відстежувати ці метрики на панелі керування FastComments у розділі аналітики використання.

---