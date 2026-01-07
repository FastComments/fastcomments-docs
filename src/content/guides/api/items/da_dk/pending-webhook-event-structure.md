Et `PendingWebhookEvent`-objekt repræsenterer en webhook-begivenhed i kø, der afventer.

`PendingWebhookEvent`-objekter oprettes automatisk og kan ikke oprettes manuelt via API'et. De udløber også efter et år.
De kan slettes, hvilket fjerner opgaven fra køen.

Der er forskellige begivenhedstyper - tjek `eventType` (`OutboundSyncEventType`) og `type` (`OutboundSyncType`).

Et almindeligt anvendelsestilfælde for dette API er at implementere brugerdefineret overvågning. Du vil måske kalde `/count`-endpointet periodisk
for at polle det udestående antal for givne filtre.

Strukturen for `PendingWebhookEvent`-objektet er som følger:

[inline-code-attrs-start title = 'PendingWebhookEvent Struktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
