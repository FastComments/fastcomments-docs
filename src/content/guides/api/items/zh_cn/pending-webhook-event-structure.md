一个 `PendingWebhookEvent` 对象表示一个处于排队等待中的 webhook 事件。

`PendingWebhookEvent` 对象会自动创建，不能通过 API 手动创建。它们也会在一年后过期。
可以删除它们，从而将任务从队列中移除。

存在不同的事件类型 —— 请检查 `eventType` (`OutboundSyncEventType`) 和 `type` (`OutboundSyncType`)。

此 API 的一个常见用例是实现自定义监控。你可能希望定期调用 `/count` 端点，以轮询给定筛选器下的未完成数量。

`PendingWebhookEvent` 对象的结构如下：

[inline-code-attrs-start title = 'PendingWebhookEvent 结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
enum OutboundSyncEventType {
    Create: 0,
    Delete: 1,
    Update: 2
}

enum OutboundSyncType {
    /** 特定于 WordPress 的同步任务。 **/
    WP: 0,
    Webhook: 1
}

interface PendingWebhookEvent {
    id: string
    /** 与该事件关联的评论 id。 **/
    commentId: string
    /** 事件发生时的 comment 对象。我们从 2023 年 11 月开始添加这些。 **/
    comment: Comment
    /** 可能与评论关联的外部 id。 **/
    externalId: string | null
    createdAt: Date
    tenantId: string
    attemptCount: number
    /** 在第一次尝试前以及每次失败后设置。 **/
    nextAttemptAt: Date
    /** 表示这是创建、删除还是更新事件。 **/
    eventType: OutboundSyncEventType
    /** 要执行的同步类型（WordPress、调用 API 等）。 **/
    type: OutboundSyncType
    /** 与评论匹配的域。我们使用该域来选择 API 密钥。 **/
    domain: string
    /** 最近发生的错误。此类型未指定具体类型，是对发生情况的“转储”。通常它包含一个具有 statusCode、body 和 headers 映射的对象。 **/
    lastError: object | null
}
[inline-code-end]

---