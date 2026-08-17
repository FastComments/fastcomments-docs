A `PendingWebhookEvent` 物件代表一個排隊中的 webhook 事件，處於待處理狀態。

`PendingWebhookEvent` 物件會自動建立，且無法透過 API 手動建立。它們也會在一年後過期。  
可以刪除它們，這會將任務從佇列中移除。

有不同的事件類型 - 請檢查 `eventType`（`OutboundSyncEventType`）和 `type`（`OutboundSyncType`）。

此 API 的常見使用情境是實作自訂監控。您可能會定期呼叫 `/count` 端點，以根據給定的篩選條件查詢未處理的計數。

`PendingWebhookEvent` 物件的結構如下：

[inline-code-attrs-start title = 'PendingWebhookEvent 結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
enum OutboundSyncEventType {
    Create: 0,
    Delete: 1,
    Update: 2
}

enum OutboundSyncType {
    /** 針對 WordPress 的同步任務。 **/
    WP: 0,
    Webhook: 1
}

interface PendingWebhookEvent {
    id: string
    /** 與事件相關聯的評論 ID。 **/
    commentId: string
    /** 事件發生時的評論物件。我們於 2023 年 11 月開始加入此欄位。 **/
    comment: Comment
    /** 可能與評論相關聯的外部 ID。 **/
    externalId: string | null
    createdAt: Date
    tenantId: string
    attemptCount: number
    /** 在首次嘗試之前以及每次失敗後設定。 **/
    nextAttemptAt: Date
    /** 此事件是建立、刪除或更新... **/
    eventType: OutboundSyncEventType
    /** 要執行的同步類型（WordPress、呼叫 API 等）。 **/
    type: OutboundSyncType
    /** 與評論匹配的網域。我們使用此網域來選擇 API 金鑰。 **/
    domain: string
    /** 最近發生的錯誤。此類型未定型，為發生情況的「轉儲」。通常包含具有 statusCode、body 以及 headers 映射的物件。 **/
    lastError: object | null
}
[inline-code-end]