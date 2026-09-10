Webhooks 也可以透過 REST API 進行管理。這就是像 Zapier 這樣的整合在不觸碰儀表板的情況下訂閱評論事件的方式，且它遵循 REST Hooks 模式：訂閱、接收事件、取消訂閱。

API 訂閱與儀表板中設定的 webhook 共同存在。評論事件會傳送至其域名的儀表板 webhook 以及每個符合條件的 API 訂閱，各自作為獨立的傳遞。每個事件沒有單一訂閱者的限制。

## 驗證

每個請求都需要在 `x-api-key` 標頭（或 `API_KEY` 查詢參數）中提供您的 API 金鑰，並在 `tenantId` 查詢參數中提供您的租戶 ID。這兩者皆可在儀表板的 API Secret 頁面上看到。

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
| `domain` | 否 | 您帳戶設定中的一個域名。預設為 `*`，會接收所有域名的事件。 |
| `method` | 否 | `POST`（預設）、`PUT` 或 `DELETE`。 |

回應中包含訂閱資訊：

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

再次以相同的 URL、相同的事件與域名訂閱時，會回傳已存在的訂閱，而不是建立重複的訂閱，讓客戶端可以安全地重試。每個租戶最多可有 50 個 API 訂閱。

## 列表

```
GET https://fastcomments.com/api/v1/webhooks?tenantId=YOUR_TENANT_ID
```

回傳該租戶的所有 webhook，包括在儀表板中管理的（`"source": "dashboard"`）。可使用 `event`、`domain` 或 `source` 進行過濾。

## 取消訂閱

```
DELETE https://fastcomments.com/api/v1/webhooks/SUBSCRIPTION_ID?tenantId=YOUR_TENANT_ID
```

刪除訂閱同時會丟棄仍在佇列中的任何事件。只有透過 API 建立的訂閱才能以此方式刪除。儀表板的 webhook 需在 Webhooks 頁面上編輯。

## 負載與簽名

傳遞使用與儀表板 webhook 相同的負載（請參閱資料結構），並以相同的 HMAC 方案簽名（請參閱安全性與 API 令牌）。API 訂閱永遠不會收到舊版的 `token` 標頭，因此請改為驗證 `X-FastComments-Signature` 標頭。

## 回應 410 Gone

如果 API 訂閱的端點回應 HTTP `410 Gone`，FastComments 會將其視為取消訂閱：訂閱會被刪除，佇列中的事件也會一起刪除，且不會再嘗試傳遞。儀表板中設定的 webhook 永不會自動刪除；對於它們而言，410 只是一般的失敗。其他任何失敗狀態都會重試，最終會停用該 webhook，如「運作方式與重試處理」所述。

## 儀表板

API 訂閱會列在 Webhooks 頁面上，顯示於其建立時所屬的域名下，管理員可在此停用、重新啟用或刪除它們。