---
Objekat `PendingWebhookEvent` predstavlja webhook događaj koji stoji u redu i čeka obradu.

`PendingWebhookEvent` objekti se kreiraju automatski i ne mogu se ručno kreirati putem API-ja. Takođe ističu nakon jedne godine.
Mogu se obrisati što uklanja zadatak iz reda.

Postoje različiti tipovi događaja - proverite `eventType` (`OutboundSyncEventType`) i `type` (`OutboundSyncType`).

Uobičajen slučaj upotrebe ovog API-ja je implementacija prilagođenog nadzora. Možda ćete želeti da pozivate `/count` endpoint periodično
da biste ispitivali broj stavki na čekanju za date filtere.

Struktura `PendingWebhookEvent` objekta je sledeća:

[inline-code-attrs-start title = 'Struktura objekta PendingWebhookEvent'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
enum OutboundSyncEventType {
    Create: 0,
    Delete: 1,
    Update: 2
}

enum OutboundSyncType {
    /** Zadatak sinhronizacije specifičan za WordPress. **/
    WP: 0,
    Webhook: 1
}

interface PendingWebhookEvent {
    id: string
    /** ID komentara povezan sa događajem. **/
    commentId: string
    /** Objekat komentara za događaj u trenutku događaja. Počeli smo da ih dodajemo u novembru 2023. **/
    comment: Comment
    /** Spoljni id koji može biti povezan sa komentarom. **/
    externalId: string | null
    createdAt: Date
    tenantId: string
    attemptCount: number
    /** Podešava se pre prvog pokušaja i nakon svakog neuspeha. **/
    nextAttemptAt: Date
    /** Da li je u pitanju događaj kreiranja, brisanja ili ažuriranja... **/
    eventType: OutboundSyncEventType
    /** Tip sinhronizacije koja treba da se izvrši (WordPress, poziv API-ja, itd.). **/
    type: OutboundSyncType
    /** Domen koji je odgovarao komentaru. Koristimo ovaj domen za izbor API ključa. **/
    domain: string
    /** Poslednja greška koja se desila. Ovaj tip nije tipiziran i predstavlja "dump" onoga što se desilo. Obično sadrži objekat sa statusCode, body, i mapom headers. **/
    lastError: object | null
}
[inline-code-end]

---