Un objeto `PendingWebhookEvent` representa un evento de webhook en cola que está pendiente.

Los objetos `PendingWebhookEvent` se crean automáticamente y no pueden crearse manualmente vía la API. También expiran después de un año.
Pueden eliminarse lo que remueve la tarea de la cola.

Hay diferentes tipos de eventos - consulte `eventType` (`OutboundSyncEventType`) y `type` (`OutboundSyncType`).

Un caso de uso común para esta API es implementar monitoreo personalizado. Puede querer llamar al endpoint `/count` periódicamente
para sondear el conteo pendiente para filtros dados.

La estructura del objeto `PendingWebhookEvent` es la siguiente:

[inline-code-attrs-start title = 'Estructura de PendingWebhookEvent'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
