---
A `PendingWebhookEvent` object represents a queued webhook event that is pending.

`PendingWebhookEvent` objects are created automatically and cannot be manually created via the API. They also expire after one year.
They can be deleted which removes the task from the queue.

There are different event types - check `eventType` (`OutboundSyncEventType`) and `type` (`OutboundSyncType`).

A common use case for this API is to implement custom monitoring. You may want to call the `/count` endpoint periodically
to poll the outstanding count for given filters.

The structure for the `PendingWebhookEvent` object is as follows:

[inline-code-attrs-start title = 'PendingWebhookEvent 結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
enum OutboundSyncEventType {
    Create: 0,
    Delete: 1,
    Update: 2
}

enum OutboundSyncType {
    /** WordPress 專用的同步任務。 **/
    WP: 0,
    Webhook: 1
}

interface PendingWebhookEvent {
    id: string
    /** 與事件相關聯的評論 id。 **/
    commentId: string
    /** 事件發生時對應的評論物件。我們從 2023 年 11 月開始加入這些資訊。 **/
    comment: Comment
    /** 可能與評論相關聯的外部 id。 **/
    externalId: string | null
    createdAt: Date
    tenantId: string
    attemptCount: number
    /** 在第一次嘗試前以及每次失敗後設定。 **/
    nextAttemptAt: Date
    /** 表示這是建立、刪除或更新事件之一... **/
    eventType: OutboundSyncEventType
    /** 要執行的同步類型（WordPress、呼叫 API 等）。 **/
    type: OutboundSyncType
    /** 與評論相符合的網域。我們使用此網域來選擇 API 金鑰。 **/
    domain: string
    /** 最近發生的錯誤。此欄位未具型別，為對發生內容的「傾印」。通常包含一個具有 statusCode、body 與 headers 地圖的物件。 **/
    lastError: object | null
}
[inline-code-end]

---