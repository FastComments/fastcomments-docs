### API 總覽

Image Chat 提供三個 REST API 端點，用於以程式方式管理圖片對話。這些端點讓您無需使用瀏覽器小工具即可擷取、建立及刪除標記。

所有 API 端點都需要驗證，並遵循標準的 FastComments API 模式。這些是透過瀏覽器 Cookie（而非 API 金鑰）進行驗證的公開端點。

### 基底 URL

所有 Image Chat API 端點都位於：

```
https://fastcomments.com/comment-image-chats
```

### 驗證

這些端點透過瀏覽器 Cookie 驗證使用者。它們不使用 API 金鑰。使用者必須在瀏覽器中登入 FastComments 才能存取這些端點。

### 取得所有對話

擷取特定圖片的所有圖片對話。

#### 端點

```
GET /comment-image-chats/:tenantId?urlId=<urlId>
```

#### 參數

`tenantId`（路徑參數，必填）是您的 FastComments Tenant ID。

`urlId`（查詢參數，必填）是您要擷取對話的圖片識別碼。

#### 回應

回應包含 API 狀態、已驗證時的目前使用者工作階段資訊、包含對話 ID、URL 與 X/Y 座標的對話陣列、清理後的 URL 識別碼、表示目前使用者是否為網站管理員或版主的旗標，以及用於即時同步的 WebSocket 連線詳細資訊，包括 `tenantIdWS`、`urlIdWS` 與 `userIdWS`。

#### 範例請求

```bash
curl "https://fastcomments.com/comment-image-chats/demo?urlId=my-product-image" \
  --cookie "your-session-cookie"
```

#### 範例回應

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

### 建立對話

為圖片上的特定位置建立新的圖片對話。

#### 端點

```
POST /comment-image-chats/:tenantId
```

#### 參數

`tenantId`（路徑參數，必填）是您的 FastComments Tenant ID。

請求主體必須為 JSON，並包含以下必填欄位。

`urlId`（字串，必填）是基礎頁面識別碼。

`windowUrlId`（字串，必填）是結合圖片來源與座標的 URL，例如 `my-page:/images/photo.jpg:25.5:30.2`。

`pageTitle`（字串，必填）是頁面的標題。

`src`（字串，必填）是圖片來源 URL。

`x`（數字，必填）是以百分比表示的 X 座標（0-100）。

`y`（數字，必填）是以百分比表示的 Y 座標（0-100）。

`createdFromCommentId`（字串，必填）是發起此對話的留言 ID。

`broadcastId`（字串，必填）是用於即時同步以防止回音效應的 UUID。

#### 回應

回應包含 API 狀態與新建對話的 ID。

#### 範例請求

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

#### 範例回應

```json
{
  "status": "success",
  "createdChatId": "conv789"
}
```

### 刪除對話

刪除現有的圖片對話。此端點需要管理員或版主權限，或者該對話必須是由已驗證的使用者所建立。

#### 端點

```
DELETE /comment-image-chats/:tenantId/:chatId
```

#### 參數

`tenantId`（路徑參數，必填）是您的 FastComments Tenant ID。

`chatId`（路徑參數，必填）是要刪除的對話 ID。

`broadcastId`（請求主體，必填）是用於即時同步的 UUID。

#### 範例請求

```bash
curl -X DELETE "https://fastcomments.com/comment-image-chats/demo/conv789" \
  --cookie "your-session-cookie" \
  -H "Content-Type: application/json" \
  -d '{
    "broadcastId": "550e8400-e29b-41d4-a716-446655440001"
  }'
```

#### 範例回應

```json
{
  "status": "success"
}
```

### 座標系統

X 與 Y 座標以圖片尺寸的百分比儲存。X 代表距離左邊緣的水平位置（0 = 左邊緣，100 = 右邊緣）。Y 代表距離頂部邊緣的垂直位置（0 = 頂部邊緣，100 = 底部邊緣）。

這些百分比值可以包含小數以提升精確度。例如，x: 25.5 表示距離圖片左邊緣 25.5%。

### 速率限制

這些端點受標準 FastComments API 速率限制約束。速率限制中介軟體按租戶套用，以防止濫用，同時允許正常的使用模式。

### 錯誤回應

所有端點都會回傳標準的 HTTP 狀態碼。400 回應表示請求參數無效。401 回應表示驗證失敗。403 回應表示權限不足。404 回應表示找不到對話。429 回應表示超出速率限制。

錯誤回應包含帶有詳細資訊的 JSON 主體：

```json
{
  "status": "error",
  "message": "Conversation not found"
}
```

### 用量追蹤

建立對話會增加您的 `conversationCreateCount` 用量指標。所有 WebSocket 同步活動會增加 `pubSubMessageCount` 與 `pubSubBandwidth`。您可以在 FastComments 儀表板的用量分析中監控這些指標。