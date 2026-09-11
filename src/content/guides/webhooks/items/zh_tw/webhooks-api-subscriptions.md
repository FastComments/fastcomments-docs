Webhooks 也可以透過 REST API 進行管理。這就是像 Zapier 這樣的整合在不觸碰儀表板的情況下訂閱評論事件的方式，且它遵循 REST Hooks 模式：訂閱、接收事件、取消訂閱。

API 訂閱與儀表板中設定的 Webhooks 同時存在。評論事件會傳送給每一個符合其 domain 的 webhook，各自作為獨立的傳遞，無論 webhook 是以何種方式建立。

## Authentication

每個請求都需要在 `x-api-key` 標頭（或 `API_KEY` 查詢參數）中提供您的 API 金鑰，並在 `tenantId` 查詢參數中提供您的租戶 ID。兩者皆可在儀表板的 API Secret 頁面上看到。

## Subscribe

```
POST https://fastcomments.com/api/v1/webhooks?tenantId=YOUR_TENANT_ID
x-api-key: YOUR_API_KEY
Content-Type: application/json

{
    "url": "https://hooks.zapier.com/hooks/catch/123/abc",
    "event": "comment-created"
}
```

| 欄位 | 必填 | 說明 |
|-------|----------|-------------|
| `url` | 是 | 絕對的 http 或 https URL。 |
| `event` | 是 | `comment-created`、`comment-updated` 或 `comment-deleted`。 |
| `domain` | 否 | 您帳號設定中的一個 domain。預設為 `*`，會接收所有 domain 的事件。 |
| `method` | 否 | `POST`（預設）、`PUT` 或 `DELETE`。 |

回應中會包含此訂閱資訊：

```json
{
    "status": "success",
    "webhook": {
        "id": "66f1c4c1e7a2b3d4f5a6b7c8",
        "url": "https://hooks.zapier.com/hooks/catch/123/abc",
        "event": "comment-created",
        "domain": "*",
        "method": "POST",
        "source": "api",
        "enabled": true,
        "createdAt": "2026-09-08T12:00:00.000Z"
    }
}
```

將相同的 URL、相同的事件與 domain 再次訂閱時，會回傳已存在的訂閱，而不是建立重複項目，讓客戶端可以安全地重試。每個租戶最多可有 50 個 API 訂閱。

## List

```
GET https://fastcomments.com/api/v1/webhooks?tenantId=YOUR_TENANT_ID
```

返回該租戶的所有 webhook，包括儀表板中管理的（`"source": "dashboard"`）。可使用 `event`、`domain` 或 `source` 進行過濾。

## Unsubscribe

```
DELETE https://fastcomments.com/api/v1/webhooks/SUBSCRIPTION_ID?tenantId=YOUR_TENANT_ID
```

刪除訂閱同時會丟棄仍在佇列中的任何事件。只有透過 API 建立的訂閱才能以此方式刪除；儀表板的 webhook，或是帳號中不存在的 ID，會回傳 `404` 並帶有 `not-found` 錯誤碼。儀表板的 webhook 可在 Webhooks 頁面上編輯。

## Payloads and signing

傳遞使用與儀表板 webhook 相同的 payload（請參閱 Data Structures），並以相同的 HMAC 方式簽名（請參閱 Security & API Tokens）。API 訂閱永不會收到舊版的 `token` 標頭，請改為驗證 `X-FastComments-Signature` 標頭。

## Sample payloads

```
GET https://fastcomments.com/api/v1/webhooks/sample-payloads?tenantId=YOUR_TENANT_ID&event=comment-created&limit=3
```

返回帳號最近的評論，完全符合傳遞的資料結構，讓整合在第一個事件到達前就能顯示真實的範例資料。`event` 為可選項且僅作驗證，因為每個事件都會傳遞相同的評論物件。`limit` 預設為 3，接受 1 到 10 的值。耗費 2 個 API 點數。

```json
{
    "status": "success",
    "payloads": [
        {
            "id": "66f1c4c1e7a2b3d4f5a6b7c8",
            "urlId": "https://example.com/blog/hello-world",
            "commenterName": "Jane Reader",
            "comment": "Great article!",
            "date": "2026-09-08T12:00:00.000Z",
            "approved": true
        }
    ]
}
```

## Responding with 410 Gone

如果 API 訂閱的端點回應 HTTP `410 Gone`，FastComments 會將其視為取消訂閱：訂閱會被刪除，佇列中的事件也會一起清除，且不會再嘗試傳遞。儀表板中設定的 webhook 永不會自動刪除；對它們而言 410 只是普通的失敗。其他任何失敗狀態都會重試，最終會停用該 webhook，詳情請參閱 How it Works & Handling Retries。

## Dashboard

API 訂閱會在 Webhooks 列表中顯示來源 **API**，管理員可以編輯、停用、重新啟用或刪除它們。