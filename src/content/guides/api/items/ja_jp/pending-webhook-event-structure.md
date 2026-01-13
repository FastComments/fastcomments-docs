A `PendingWebhookEvent` object represents a queued webhook event that is pending.

`PendingWebhookEvent` オブジェクトは自動的に作成され、API 経由で手動作成することはできません。これらはまた 1 年後に期限切れになります。
それらは削除可能で、削除するとキューからタスクが取り除かれます。

異なるイベントタイプがあります — `eventType`（`OutboundSyncEventType`）と `type`（`OutboundSyncType`）を確認してください。

この API の一般的なユースケースはカスタム監視を実装することです。指定したフィルターに対する未処理数をポーリングするために、`/count` エンドポイントを定期的に呼び出すことがあるでしょう。

The structure for the `PendingWebhookEvent` object is as follows:

[inline-code-attrs-start title = 'PendingWebhookEvent の構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
enum OutboundSyncEventType {
    Create: 0,
    Delete: 1,
    Update: 2
}

enum OutboundSyncType {
    /** WordPress-specific sync task. **/
    WP: 0,
    Webhook: 1
}

interface PendingWebhookEvent {
    id: string
    /** The comment id associated with the event. **/
    commentId: string
    /** The comment object for the event at the time of the event. We started adding these in Nov 2023. **/
    comment: Comment
    /** An external id that may be associated with the comment. **/
    externalId: string | null
    createdAt: Date
    tenantId: string
    attemptCount: number
    /** Set before first attempt, and after every failure. **/
    nextAttemptAt: Date
    /** Whether this is a creation, deletion, or update event... **/
    eventType: OutboundSyncEventType
    /** The type of sync to perform (WordPress, call API, etc). **/
    type: OutboundSyncType
    /** The domain that matched the comment. We use this domain to pick the API key. **/
    domain: string
    /** The last error that occurred. This type is untyped and is a "dump" of whatever happened. Usually it contains an object with statusCode, body, and a headers map. **/
    lastError: object | null
}
[inline-code-end]