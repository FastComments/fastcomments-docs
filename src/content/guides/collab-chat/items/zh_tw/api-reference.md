### API Overview

Collab Chat 提供三個用於以程式化方式管理文字對話的 REST API 端點。這些端點可讓您在不使用瀏覽器外掛程式的情況下檢索、建立和刪除註解。

這些為公開端點，透過瀏覽器 cookie 驗證使用者。它們不使用 API 金鑰。使用者必須在瀏覽器中登入 FastComments 才能存取這些端點。

### Base URL

All Collab Chat API endpoints are under:

[inline-code-attrs-start title = '基本 URL'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/comment-collab-chats
[inline-code-end]

### Authentication

These endpoints authenticate users via browser cookies. They do not use API keys. Users must be logged into FastComments in their browser to access these endpoints.

### Get All Conversations

Retrieve all text conversations for a specific page.

#### Endpoint

[inline-code-attrs-start title = 'GET 端點'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
GET /comment-collab-chats/:tenantId?urlId=<urlId>
[inline-code-end]

#### Parameters

`tenantId` (path parameter, required) 是您的 FastComments Tenant ID。

`urlId` (query parameter, required) 是您要檢索對話的頁面識別碼。

#### Response

回應包含 API 狀態、若已驗證則的當前使用者會話資訊、一個包含對話 ID、URL 和文字範圍的對話陣列、清理後的 URL 識別碼、一個表示當前使用者是否為網站管理員或版主的旗標，以及用於即時同步的 WebSocket 連線細節，包括 `tenantIdWS`、`urlIdWS` 和 `userIdWS`。

#### Example Request

[inline-code-attrs-start title = 'GET 範例請求'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl "https://fastcomments.com/comment-collab-chats/demo?urlId=my-article-page" \
  --cookie "your-session-cookie"
[inline-code-end]

#### Example Response

[inline-code-attrs-start title = 'GET 範例回應'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

### Create Conversation

Create a new text conversation for a specific text selection.

#### Endpoint

[inline-code-attrs-start title = 'POST 端點'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
POST /comment-collab-chats/:tenantId
[inline-code-end]

#### Parameters

`tenantId` (path parameter, required) 是您的 FastComments Tenant ID。

The request body must be JSON and include these required fields.

`urlId` (string, required) 是基本頁面識別碼。

`urlIdWithRange` (string, required) 是結合文字範圍的 URL，例如 `my-page:p:0:15,0:45{abc123}`。

`pageTitle` (string, required) 是頁面的標題。

`selector` (string, required) 是包含已選取文字之元素的 DOM 路徑。

`range` (string, required) 是序列化的文字範圍，格式為 `startOffset:endOffset,startOffset:endOffset{checksum}`。

`createdFromCommentId` (string, required) 是啟動此聊天的評論 ID。

`broadcastId` (string, required) 是用於即時同步以避免回波效應的 UUID。

#### Response

回應包含 API 狀態以及新建立對話的 ID。

#### Example Request

[inline-code-attrs-start title = 'POST 範例請求'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
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

#### Example Response

[inline-code-attrs-start title = 'POST 範例回應'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success",
  "createdChatId": "conv789"
}
[inline-code-end]

### Delete Conversation

Delete an existing text conversation. This endpoint requires admin or moderator permissions, or the conversation must have been created by the authenticated user.

#### Endpoint

[inline-code-attrs-start title = 'DELETE 端點'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
DELETE /comment-collab-chats/:tenantId/:chatId
[inline-code-end]

#### Parameters

`tenantId` (path parameter, required) 是您的 FastComments Tenant ID。

`chatId` (path parameter, required) 是要刪除的對話 ID。

`broadcastId` (request body, required) 是用於即時同步的 UUID。

#### Example Request

[inline-code-attrs-start title = 'DELETE 範例請求'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl -X DELETE "https://fastcomments.com/comment-collab-chats/demo/conv789" \
  --cookie "your-session-cookie" \
  -H "Content-Type: application/json" \
  -d '{
    "broadcastId": "550e8400-e29b-41d4-a716-446655440001"
  }'
[inline-code-end]

#### Example Response

[inline-code-attrs-start title = 'DELETE 範例回應'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success"
}
[inline-code-end]

### Rate Limiting

These endpoints are subject to standard FastComments API rate limiting. The rate limit middleware applies per-tenant to prevent abuse while allowing normal usage patterns.

### Error Responses

All endpoints return standard HTTP status codes. A 400 response indicates invalid request parameters. A 401 response means authentication failed. A 403 response indicates insufficient permissions. A 404 response means the conversation was not found. A 429 response indicates rate limit exceeded.

Error responses include a JSON body with details:

[inline-code-attrs-start title = '錯誤回應範例'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "error",
  "message": "Conversation not found"
}
[inline-code-end]

### Usage Tracking

Creating conversations increments your `conversationCreateCount` usage metric. All WebSocket sync activity increments `pubSubMessageCount` and `pubSubBandwidth`. You can monitor these metrics in your FastComments dashboard under usage analytics.

---