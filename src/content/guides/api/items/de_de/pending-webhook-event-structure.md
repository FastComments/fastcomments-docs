Ein `PendingWebhookEvent`-Objekt repräsentiert ein in der Warteschlange befindliches Webhook-Ereignis, das aussteht.

`PendingWebhookEvent`-Objekte werden automatisch erstellt und können nicht manuell über die API erstellt werden. Sie laufen auch nach einem Jahr ab.
Sie können gelöscht werden, wodurch die Aufgabe aus der Warteschlange entfernt wird.

Es gibt verschiedene Ereignistypen - prüfen Sie `eventType` (`OutboundSyncEventType`) und `type` (`OutboundSyncType`).

Ein häufiger Anwendungsfall für diese API ist die Implementierung von benutzerdefiniertem Monitoring. Sie möchten vielleicht den `/count`-Endpunkt regelmäßig aufrufen,
um die ausstehende Anzahl für gegebene Filter abzufragen.

Die Struktur des `PendingWebhookEvent`-Objekts ist wie folgt:

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
