Un oggetto `PendingWebhookEvent` rappresenta un evento webhook in coda che è in attesa di elaborazione.

Gli oggetti `PendingWebhookEvent` vengono creati automaticamente e non possono essere creati manualmente tramite l'API. Inoltre scadono dopo un anno.
Possono essere eliminati, il che rimuove l'attività dalla coda.

Esistono diversi tipi di evento - controllare `eventType` (`OutboundSyncEventType`) e `type` (`OutboundSyncType`).

Un caso d'uso comune per questa API è l'implementazione di un monitoraggio personalizzato. Potrebbe essere necessario chiamare periodicamente l'endpoint `/count`
per interrogare il numero di eventi in sospeso in base ai filtri specificati.

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
    /** L'oggetto commento per l'evento al momento dell'evento. Abbiamo iniziato ad aggiungerli a novembre 2023. **/
    comment: Comment
    /** Un id esterno che può essere associato al commento. **/
    externalId: string | null
    createdAt: Date
    tenantId: string
    attemptCount: number
    /** Impostato prima del primo tentativo e dopo ogni fallimento. **/
    nextAttemptAt: Date
    /** Indica se si tratta di un evento di creazione, eliminazione o aggiornamento... **/
    eventType: OutboundSyncEventType
    /** Il tipo di sincronizzazione da eseguire (WordPress, chiamata API, ecc). **/
    type: OutboundSyncType
    /** Il dominio corrispondente al commento. Usiamo questo dominio per selezionare la chiave API. **/
    domain: string
    /** L'ultimo errore verificatosi. Questo tipo è non tipizzato ed è un "dump" di quanto accaduto. Di solito contiene un oggetto con statusCode, body e una mappa headers. **/
    lastError: object | null
}
[inline-code-end]