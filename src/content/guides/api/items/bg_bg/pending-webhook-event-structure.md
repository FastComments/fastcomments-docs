Обектът `PendingWebhookEvent` представлява webhook събитие в опашката, което чака обработка.

Обектите `PendingWebhookEvent` се създават автоматично и не могат да бъдат създадени ръчно чрез API. Те също изтичат след една година.
Могат да бъдат изтрити, което премахва задачата от опашката.

Има различни типове събития - проверете `eventType` (`OutboundSyncEventType`) и `type` (`OutboundSyncType`).

Често срещан случай на употреба за този API е внедряването на персонализиран мониторинг. Може да искате да извикате крайната точка `/count` периодично,
за да проверявате броя на чакащите събития за дадени филтри.

Структурата на обекта `PendingWebhookEvent` е следната:

[inline-code-attrs-start title = 'Структура на PendingWebhookEvent'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
