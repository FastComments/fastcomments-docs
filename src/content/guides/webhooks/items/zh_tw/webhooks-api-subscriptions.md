Webhooks 也可以透過 REST API 管理。像 Zapier 這類整合服務就是透過這種方式訂閱留言事件，而無需操作儀表板，並且遵循 REST Hooks 模式：訂閱、接收事件、取消訂閱。

API 訂閱與儀表板中設定的 webhooks 並存。留言事件會傳送給每個符合其網域的 webhook，每個都是獨立的傳送，無論該 webhook 是以何種方式建立的。

## 驗證

每個請求都需要在 `x-api-key` 標頭(或 `API_KEY` 查詢參數)中提供您的 API Key,並在 `tenantId` 查詢參數中提供您的租戶 ID。兩者都顯示在儀表板的 API Secret 頁面上。

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
| `url` | 是 | 絕對的 http 或 https URL。 |
| `event` | 是 | `comment-created`、`comment-updated` 或 `comment-deleted`。 |
| `domain` | 否 | 您帳戶設定中的網域。預設為 `*`,會接收所有網域的事件。 |
| `method` | 否 | `POST`(預設)、`PUT` 或 `DELETE`。 |

回應包含該訂閱:

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

將相同的 URL 再次訂閱相同的事件與網域時,會回傳現有的訂閱而非建立重複項,因此用戶端可以安全地重試。每個租戶最多可有 50 個 API 訂閱。

## 列出

```
GET https://fastcomments.com/api/v1/webhooks?tenantId=YOUR_TENANT_ID
```

回傳該租戶的所有 webhook,包括在儀表板中管理的那些(`"source": "dashboard"`)。可以使用 `event`、`domain` 或 `source` 進行篩選。

## 取消訂閱

```
DELETE https://fastcomments.com/api/v1/webhooks/SUBSCRIPTION_ID?tenantId=YOUR_TENANT_ID
```

刪除訂閱時,也會一併捨棄仍在佇列中的事件。只有透過 API 建立的訂閱才能以這種方式刪除;儀表板 webhook,或您帳戶中不存在的 id,會回應 `404` 與代碼 `not-found`。儀表板 webhook 請在 Webhooks 頁面上編輯。

## 負載與簽署

傳送內容使用與儀表板 webhook 相同的負載(參閱 Data Structures),並以相同的 HMAC 方案簽署(參閱 Security & API Tokens)。API 訂閱永遠不會收到舊版的 `token` 標頭,因此請改為驗證 `X-FastComments-Signature` 標頭。

## 範例負載

```
GET https://fastcomments.com/api/v1/webhooks/sample-payloads?tenantId=YOUR_TENANT_ID&event=comment-created&limit=3
```

以傳送內容的確切格式回傳帳戶最近的留言,讓整合服務可以在第一個事件到達之前顯示真實的範例資料。`event` 為選填且僅做驗證,因為每個事件都傳送相同的留言物件。`limit` 預設為 3,接受 1 到 10。花費 2 個 API 點數。

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

## 以 410 Gone 回應

如果 API 訂閱的端點以 HTTP `410 Gone` 回應,FastComments 會將其視為取消訂閱:該訂閱會連同其佇列中的事件一起刪除,並且不再嘗試任何後續傳送。在儀表板中設定的 webhooks 永遠不會被自動刪除;對它們而言,410 只是一般的失敗。任何其他失敗狀態會被重試,並最終停用該 webhook,如 How it Works & Handling Retries 中所述。

## 儀表板

API 訂閱會以來源 **API** 顯示在 Webhooks 清單中,管理員可以在那裡編輯、停用、重新啟用或刪除它們。