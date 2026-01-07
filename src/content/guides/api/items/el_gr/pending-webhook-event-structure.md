Ένα αντικείμενο `PendingWebhookEvent` αντιπροσωπεύει ένα γεγονός webhook στην ουρά που εκκρεμεί.

Τα αντικείμενα `PendingWebhookEvent` δημιουργούνται αυτόματα και δεν μπορούν να δημιουργηθούν χειροκίνητα μέσω του API. Επίσης λήγουν μετά από ένα έτος.
Μπορούν να διαγραφούν, πράγμα που αφαιρεί την εργασία από την ουρά.

Υπάρχουν διαφορετικοί τύποι γεγονότων - ελέγξτε τα `eventType` (`OutboundSyncEventType`) και `type` (`OutboundSyncType`).

Μια συνήθης περίπτωση χρήσης για αυτό το API είναι η υλοποίηση προσαρμοσμένης παρακολούθησης. Μπορεί να θέλετε να καλείτε το endpoint `/count` περιοδικά
για να ελέγχετε τον εκκρεμή αριθμό για δεδομένα φίλτρα.

Η δομή για το αντικείμενο `PendingWebhookEvent` είναι η ακόλουθη:

[inline-code-attrs-start title = 'Δομή PendingWebhookEvent'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
