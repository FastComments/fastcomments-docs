### API 總覽

Collab Chat 提供三個 REST API 端點，用於以程式方式管理文字對話。這些端點讓您無需使用瀏覽器小工具即可擷取、建立和刪除標註。

這些是透過瀏覽器 Cookie 對使用者進行身分驗證的公開端點。它們不使用 API 金鑰。使用者必須在其瀏覽器中登入 FastComments 才能存取這些端點。

### 基底 URL

所有 Collab Chat API 端點都位於：

[inline-code-attrs-start title = '基底 URL'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/comment-collab-chats
[inline-code-end]

### 身分驗證

這些端點透過瀏覽器 Cookie 對使用者進行身分驗證。它們不使用 API 金鑰。使用者必須在其瀏覽器中登入 FastComments 才能存取這些端點。

### 取得所有對話

擷取特定頁面的所有文字對話。

#### 端點

[inline-code-attrs-start title = 'GET 端點'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
GET /comment-collab-chats/:tenantId?urlId=<urlId>
[inline-code-end]

#### 參數

`tenantId`（路徑參數，必填）是您的 FastComments 租戶 ID。

`urlId`（查詢參數，必填）是您要擷取對話的頁面識別碼。

#### 回應

回應包含 API 狀態、已驗證時的目前使用者工作階段資訊、包含對話 ID、URL 與文字範圍的對話陣列、清理後的 URL 識別碼、表示目前使用者是否為網站管理員或版主的旗標，以及用於即時同步的 WebSocket 連線詳細資訊，包括 `tenantIdWS`、`urlIdWS` 和 `userIdWS`。

#### 請求範例

[inline-code-attrs-start title = 'GET 請求範例'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl "https://fastcomments.com/comment-collab-chats/demo?urlId=my-article-page" \
  --cookie "your-session-cookie"
[inline-code-end]

#### 回應範例

[inline-code-attrs-start title = 'GET 回應範例'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

### 建立對話

為特定文字選取範圍建立新的文字對話。

#### 端點

[inline-code-attrs-start title = 'POST 端點'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
POST /comment-collab-chats/:tenantId
[inline-code-end]

#### 參數

`tenantId`（路徑參數，必填）是您的 FastComments 租戶 ID。

請求主體必須是 JSON，並包含以下必填欄位。

`urlId`（字串，必填）是基底頁面識別碼。

`urlIdWithRange`（字串，必填）是與文字範圍結合的 URL，例如 `my-page:p:0:15,0:45{abc123}`。

`pageTitle`（字串，必填）是頁面的標題。

`selector`（字串，必填）是包含所選文字之元素的 DOM 路徑。

`range`（字串，必填）是格式為 `startOffset:endOffset,startOffset:endOffset{checksum}` 的序列化文字範圍。

`createdFromCommentId`（字串，必填）是發起此對話的留言 ID。

`broadcastId`（字串，必填）是用於即時同步以防止回音效應的 UUID。

#### 回應

回應包含 API 狀態以及新建對話的 ID。

#### 請求範例

[inline-code-attrs-start title = 'POST 請求範例'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
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

#### 回應範例

[inline-code-attrs-start title = 'POST 回應範例'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success",
  "createdChatId": "conv789"
}
[inline-code-end]

### 刪除對話

刪除現有的文字對話。此端點需要管理員或版主權限，或者該對話必須是由已驗證的使用者所建立。

#### 端點

[inline-code-attrs-start title = 'DELETE 端點'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
DELETE /comment-collab-chats/:tenantId/:chatId
[inline-code-end]

#### 參數

`tenantId`（路徑參數，必填）是您的 FastComments 租戶 ID。

`chatId`（路徑參數，必填）是要刪除的對話 ID。

`broadcastId`（請求主體，必填）是用於即時同步的 UUID。

#### 請求範例

[inline-code-attrs-start title = 'DELETE 請求範例'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl -X DELETE "https://fastcomments.com/comment-collab-chats/demo/conv789" \
  --cookie "your-session-cookie" \
  -H "Content-Type: application/json" \
  -d '{
    "broadcastId": "550e8400-e29b-41d4-a716-446655440001"
  }'
[inline-code-end]

#### 回應範例

[inline-code-attrs-start title = 'DELETE 回應範例'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success"
}
[inline-code-end]

### 速率限制

這些端點受標準 FastComments API 速率限制約束。速率限制中介軟體按租戶套用，以防止濫用，同時允許正常的使用模式。

### 錯誤回應

所有端點都會回傳標準 HTTP 狀態碼。400 回應表示請求參數無效。401 回應表示身分驗證失敗。403 回應表示權限不足。404 回應表示找不到該對話。429 回應表示超出速率限制。

錯誤回應包含帶有詳細資訊的 JSON 主體：

[inline-code-attrs-start title = '錯誤回應範例'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "error",
  "message": "Conversation not found"
}
[inline-code-end]

### 用量追蹤

建立對話會增加您的 `conversationCreateCount` 用量指標。所有 WebSocket 同步活動會增加 `pubSubMessageCount` 和 `pubSubBandwidth`。您可以在 FastComments 儀表板的用量分析中監控這些指標。