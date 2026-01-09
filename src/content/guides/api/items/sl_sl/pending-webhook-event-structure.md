---
Objekt `PendingWebhookEvent` predstavlja webhook dogodek v čakalni vrsti.

Objekti `PendingWebhookEvent` se ustvarijo samodejno in jih ni mogoče ročno ustvariti prek API. Potečejo tudi po enem letu.
Lahko jih izbrišete, kar odstrani nalogo iz vrste.

Obstajajo različne vrste dogodkov - preverite `eventType` (`OutboundSyncEventType`) in `type` (`OutboundSyncType`).

Pogost primer uporabe tega API-ja je implementacija prilagojenega nadzora. Morda boste želeli občasno klicati `/count` končno točko
za poizvedovanje o številu nerešenih primerov za določene filtre.

Struktura objekta `PendingWebhookEvent` je naslednja:

[inline-code-attrs-start title = 'Struktura PendingWebhookEvent'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
enum OutboundSyncEventType {
    Create: 0,
    Delete: 1,
    Update: 2
}

enum OutboundSyncType {
    /** Naloga sinhronizacije specifična za WordPress. **/
    WP: 0,
    Webhook: 1
}

interface PendingWebhookEvent {
    id: string
    /** ID komentarja, povezanega z dogodkom. **/
    commentId: string
    /** Objekt komentarja za dogodek ob času dogodka. Začeli smo jih dodajati novembra 2023. **/
    comment: Comment
    /** Zunanji ID, ki je lahko povezan s komentarjem. **/
    externalId: string | null
    createdAt: Date
    tenantId: string
    attemptCount: number
    /** Nastavljeno pred prvim poskusom in po vsaki napaki. **/
    nextAttemptAt: Date
    /** Ali gre za dogodek ustvarjanja, brisanja ali posodobitve... **/
    eventType: OutboundSyncEventType
    /** Vrsta sinhronizacije, ki jo je treba izvesti (WordPress, klic API, itd.). **/
    type: OutboundSyncType
    /** Domena, ki se je ujemala s komentarjem. To domeno uporabimo za izbiro API ključa. **/
    domain: string
    /** Zadnja nastala napaka. Ta tip ni tipiziran in je "dump" tega, kar se je zgodilo. Običajno vsebuje objekt s statusCode, body, in mapo headers. **/
    lastError: object | null
}
[inline-code-end]

---