### API 概覽

Image Chat 提供三個 REST API 端點，用於以程式方式管理影像對話。這些端點讓您可以在不使用瀏覽器小工具的情況下檢索、建立和刪除標記。

所有 API 端點都需要認證，並遵循標準 FastComments API 模式。這些是透過瀏覽器 cookie 認證的公開端點，而不是使用 API 金鑰。

### Base URL

所有 Image Chat API 端點位於：

```
https://fastcomments.com/comment-image-chats
```

### Authentication

這些端點透過瀏覽器 cookie 來認證使用者。它們不使用 API 金鑰。使用者必須在其瀏覽器中登入 FastComments 才能存取這些端點。

### Get All Conversations

擷取特定圖片的所有影像對話。

#### Endpoint

```
GET /comment-image-chats/:tenantId?urlId=<urlId>
```

#### Parameters

`tenantId` (path parameter, required) 是您的 FastComments Tenant ID。

`urlId` (query parameter, required) 是您要檢索對話的圖片識別碼。

#### Response

回應包含 API 狀態、若已認證的當前使用者會話資訊、包含其 ID、URL 及 X/Y 座標的對話陣列、清理過的 URL 識別碼、一個標示目前使用者是否為網站管理員或版主的旗標，以及用於即時同步的 WebSocket 連線詳細資訊，包括 `tenantIdWS`、`urlIdWS` 和 `userIdWS`。

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

為圖片上的特定位置建立新的影像對話。

#### Endpoint

```
POST /comment-image-chats/:tenantId
```

#### Parameters

`tenantId` (path parameter, required) 是您的 FastComments Tenant ID。

請求主體必須為 JSON 並包含以下必要欄位。

`urlId` (string, required) 是基礎頁面識別碼。

`windowUrlId` (string, required) 是結合了 URL、圖片來源與座標的識別字串，例如 `my-page:/images/photo.jpg:25.5:30.2`。

`pageTitle` (string, required) 是頁面的標題。

`src` (string, required) 是圖片來源 URL。

`x` (number, required) 是 X 座標，表示百分比 (0-100)。

`y` (number, required) 是 Y 座標，表示百分比 (0-100)。

`createdFromCommentId` (string, required) 是啟動此聊天室的評論 ID。

`broadcastId` (string, required) 是用於即時同步以避免回音效應的 UUID。

#### Response

回應包含 API 狀態以及新建立對話的 ID。

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

刪除現有的影像對話。此端點需要管理員或版主權限，或該對話必須由已驗證的使用者所建立。

#### Endpoint

```
DELETE /comment-image-chats/:tenantId/:chatId
```

#### Parameters

`tenantId` (path parameter, required) 是您的 FastComments Tenant ID。

`chatId` (path parameter, required) 是要刪除的對話 ID。

`broadcastId` (request body, required) 是用於即時同步的 UUID。

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

X 和 Y 座標以影像尺寸的百分比儲存。X 表示從左邊緣開始的水平方向位置 (0 = 左邊緣，100 = 右邊緣)。Y 表示從上邊緣開始的垂直方向位置 (0 = 上邊緣，100 = 下邊緣)。

這些百分比值可以包含小數以提高精準度。例如，x: 25.5 意味著從影像左邊緣起算的 25.5%。

### Rate Limiting

這些端點受到標準 FastComments API 的速率限制。速率限制中介軟體會以每個租戶為單位進行限制，以防止濫用同時允許正常使用模式。

### Error Responses

所有端點會回傳標準的 HTTP 狀態碼。HTTP 400 表示請求參數無效。HTTP 401 表示認證失敗。HTTP 403 表示權限不足。HTTP 404 表示未找到該對話。HTTP 429 表示超過速率限制。

錯誤回應包含帶有詳細資訊的 JSON 主體：

```json
{
  "status": "error",
  "message": "Conversation not found"
}
```

### Usage Tracking

建立對話會增加您的 `conversationCreateCount` 使用量指標。所有 WebSocket 同步活動會增加 `pubSubMessageCount` 與 `pubSubBandwidth`。您可以在 FastComments 儀表板的使用量分析中監控這些指標。