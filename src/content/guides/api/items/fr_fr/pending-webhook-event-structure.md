Un objet `PendingWebhookEvent` représente un événement webhook en file d'attente qui est en attente.

Les objets `PendingWebhookEvent` sont créés automatiquement et ne peuvent pas être créés manuellement via l'API. Ils expirent également après un an.
Ils peuvent être supprimés, ce qui retire la tâche de la file d'attente.

Il existe différents types d'événements - vérifiez `eventType` (`OutboundSyncEventType`) et `type` (`OutboundSyncType`).

Un cas d'utilisation courant pour cette API est d'implémenter une surveillance personnalisée. Vous pouvez vouloir appeler l'endpoint `/count` périodiquement
pour interroger le compteur en attente pour des filtres donnés.

La structure de l'objet `PendingWebhookEvent` est la suivante :

[inline-code-attrs-start title = 'Structure de PendingWebhookEvent'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
