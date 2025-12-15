A `PendingWebhookEvent` object represents a queued webhook event that is pending.

`PendingWebhookEvent` objects are created automatically and cannot be manually created via the API. They also expire after one year.
They can be deleted which removes the task from the queue.

There are different event types - check `eventType` (`OutboundSyncEventType`) and `type` (`OutboundSyncType`).

A common use case for this API is to implement custom monitoring. You may want to call the `/count` endpoint periodically
to poll the outstanding count for given filters.

The structure for the `PendingWebhookEvent` object is as follows:

[inline-code-attrs-start title = 'PendingWebhookEvent Structure'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
