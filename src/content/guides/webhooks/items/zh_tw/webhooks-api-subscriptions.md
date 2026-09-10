Webhooks 也可以透過 REST API 進行管理。這就是像 Zapier 這樣的整合在不觸碰儀表板的情況下訂閱評論事件的方式，且它遵循 REST Hooks 模式：訂閱、接收事件、取消訂閱。

API 訂閱與儀表板中設定的 Webhooks 並存。評論事件會傳送給每個符合其域名的 webhook，每個都作為獨立的傳遞，無論 webhook 是以何種方式建立的。

## 認證

每個請求都需要在 `x-api-key` 標頭（或 `API_KEY` 查詢參數）中提供您的 API 金鑰，並在 `tenantId` 查詢參數中提供您的租戶 ID。這兩者皆顯示於儀表板的 API Secret 頁面上。

## 訂閱

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
| `url` | Yes | 絕對的 http 或 https URL。 |
| `event` | Yes | `comment-created`、`comment-updated` 或 `comment-deleted`。 |
| `domain` | No | 您帳戶設定中的一個域名。預設為 `*`，會接收所有域名的事件。 |
| `method` | No | `POST`（預設）、`PUT` 或 `DELETE`。 |

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

再次使用相同的 URL、相同的事件與域名訂閱時，會返回已存在的訂閱，而不是建立重複的訂閱，讓客戶端可以安全地重試。每個租戶最多可擁有 50 個 API 訂閱。

## 列表

```
GET https://fastcomments.com/api/v1/webhooks?tenantId=YOUR_TENANT_ID
```

返回該租戶的所有 webhook，包括在儀表板中管理的（`"source": "dashboard"`）。可使用 `event`、`domain` 或 `source` 進行篩選。

## 取消訂閱

```
DELETE https://fastcomments.com/api/v1/webhooks/SUBSCRIPTION_ID?tenantId=YOUR_TENANT_ID
```

刪除訂閱同時會丟棄仍在佇列中的任何事件。只有透過 API 建立的訂閱才能以此方式刪除。儀表板的 webhook 需在 Webhooks 頁面上編輯。

## 負載與簽名

傳遞使用與儀表板 webhook 相同的負載（請參閱資料結構），並以相同的 HMAC 方案簽名（請參閱安全性與 API 令牌）。API 訂閱永不會收到舊版的 `token` 標頭，請改為驗證 `X-FastComments-Signature` 標頭。

## 範例負載

```
GET https://fastcomments.com/api/v1/webhooks/sample-payloads?tenantId=YOUR_TENANT_ID&event=comment-created&limit=3
```

返回帳戶最近的評論，格式與傳遞的負載完全相同，讓整合在第一個事件到達前即可顯示真實的範例資料。`event` 為可選，僅作驗證，因為每個事件都傳遞相同的評論物件。`limit` 預設為 3，接受 1 到 10 的值。消耗 2 個 API 點數。

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

## 回應 410 Gone

如果 API 訂閱的端點回應 HTTP `410 Gone`，FastComments 會將其視為取消訂閱：訂閱會被刪除，佇列中的事件也會一起刪除，且不再嘗試任何傳遞。儀表板中設定的 webhook 永不會自動刪除；對於它們而言 410 只是普通的失敗。其他任何失敗狀態都會重試，最終會停用 webhook，如「運作方式與重試處理」所述。

## 儀表板

API 訂閱會在 Webhooks 列表中顯示來源為 **API**，管理員可在此編輯、停用、重新啟用或刪除它們。

---