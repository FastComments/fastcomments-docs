Een `PendingWebhookEvent`-object vertegenwoordigt een webhook-event in de wachtrij dat nog in behandeling is.

`PendingWebhookEvent`-objecten worden automatisch aangemaakt en kunnen niet handmatig via de API worden aangemaakt. Ze verlopen ook na één jaar.
Ze kunnen worden verwijderd, wat de taak uit de wachtrij verwijdert.

Er zijn verschillende eventtypes - controleer `eventType` (`OutboundSyncEventType`) en `type` (`OutboundSyncType`).

Een veelvoorkomend gebruik van deze API is het implementeren van aangepaste monitoring. U wilt mogelijk periodiek het `/count`-endpoint aanroepen om het openstaande aantal voor bepaalde filters op te vragen.

De structuur van het `PendingWebhookEvent`-object is als volgt:

[inline-code-attrs-start title = 'PendingWebhookEvent-structuur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
enum OutboundSyncEventType {
    Create: 0,
    Delete: 1,
    Update: 2
}

enum OutboundSyncType {
    /** WordPress-specifieke synchronisatietaak. **/
    WP: 0,
    Webhook: 1
}

interface PendingWebhookEvent {
    id: string
    /** De comment id die aan het event is gekoppeld. **/
    commentId: string
    /** Het comment-object voor het event op het moment van het event. We zijn begonnen deze toe te voegen in nov 2023. **/
    comment: Comment
    /** Een externe id die mogelijk aan de comment is gekoppeld. **/
    externalId: string | null
    createdAt: Date
    tenantId: string
    attemptCount: number
    /** Ingesteld vóór de eerste poging en na elke mislukking. **/
    nextAttemptAt: Date
    /** Of dit een aanmaak-, verwijder- of update-event is... **/
    eventType: OutboundSyncEventType
    /** Het type synchronisatie dat moet worden uitgevoerd (WordPress, API-aanroep, enz.). **/
    type: OutboundSyncType
    /** Het domein dat overeenkwam met de comment. We gebruiken dit domein om de API-sleutel te kiezen. **/
    domain: string
    /** De laatste fout die optrad. Dit type is niet-getypeerd en is een "dump" van wat er ook is gebeurd. Gewoonlijk bevat het een object met statusCode, body, en een headers-map. **/
    lastError: object | null
}
[inline-code-end]

---