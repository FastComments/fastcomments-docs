Un oggetto `PendingWebhookEvent` rappresenta un evento webhook accodato in attesa.

`PendingWebhookEvent` objects are created automatically and cannot be manually created via the API. They also expire after one year.
Possono essere eliminati, il che rimuove il task dalla coda.

Esistono diversi tipi di eventi - controlla `eventType` (`OutboundSyncEventType`) e `type` (`OutboundSyncType`).

Un uso comune di questa API è implementare un monitoraggio personalizzato. Potresti voler chiamare periodicamente l'endpoint `/count`
per interrogare il conteggio in sospeso per filtri specifici.

La struttura dell'oggetto `PendingWebhookEvent` è la seguente:

[inline-code-attrs-start title = 'Struttura di PendingWebhookEvent'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
enum OutboundSyncEventType {
    Create: 0,
    Delete: 1,
    Update: 2
}

enum OutboundSyncType {
    /** Attività di sincronizzazione specifica per WordPress. **/
    WP: 0,
    Webhook: 1
}

interface PendingWebhookEvent {
    id: string
    /** L'id del commento associato all'evento. **/
    commentId: string
    /** L'oggetto del commento relativo all'evento al momento dell'evento. Abbiamo iniziato ad aggiungerli a partire da Nov 2023. **/
    comment: Comment
    /** Un id esterno che può essere associato al commento. **/
    externalId: string | null
    createdAt: Date
    tenantId: string
    attemptCount: number
    /** Impostato prima del primo tentativo e dopo ogni fallimento. **/
    nextAttemptAt: Date
    /** Se si tratta di un evento di creazione, cancellazione o aggiornamento... **/
    eventType: OutboundSyncEventType
    /** Il tipo di sincronizzazione da eseguire (WordPress, chiamare l'API, ecc.). **/
    type: OutboundSyncType
    /** Il dominio che ha corrisposto al commento. Usiamo questo dominio per scegliere la chiave API. **/
    domain: string
    /** L'ultimo errore verificatosi. Questo tipo non è tipizzato ed è un "dump" di quanto accaduto. Di solito contiene un oggetto con statusCode, body, e una mappa di headers. **/
    lastError: object | null
}
[inline-code-end]