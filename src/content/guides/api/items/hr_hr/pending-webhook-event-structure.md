---
A `PendingWebhookEvent` object predstavlja webhook događaj u redu koji je na čekanju.

`PendingWebhookEvent` objects se stvaraju automatski i ne mogu se ručno stvarati putem API-ja. Također istječu nakon jedne godine.
Mogu se izbrisati, što uklanja zadatak iz reda.

Postoje različite vrste događaja - provjerite `eventType` (`OutboundSyncEventType`) i `type` (`OutboundSyncType`).

Uobičajen slučaj upotrebe ovog API-ja je implementacija prilagođenog nadzora. Možda ćete htjeti periodički pozivati endpoint `/count` 
kako biste provjeravali broj preostalih zadataka za zadane filtere.

Struktura objekta `PendingWebhookEvent` je sljedeća:

[inline-code-attrs-start title = 'Struktura objekta PendingWebhookEvent'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
enum OutboundSyncEventType {
    Create: 0,
    Delete: 1,
    Update: 2
}

enum OutboundSyncType {
    /** Zadatak sinkronizacije specifičan za WordPress. **/
    WP: 0,
    Webhook: 1
}

interface PendingWebhookEvent {
    id: string
    /** ID komentara povezan s događajem. **/
    commentId: string
    /** Objekt komentara za događaj u trenutku događaja. Počeli smo ih dodavati u studenom 2023. **/
    comment: Comment
    /** Vanjski ID koji može biti povezan s komentarom. **/
    externalId: string | null
    createdAt: Date
    tenantId: string
    attemptCount: number
    /** Postavlja se prije prvog pokušaja i nakon svakog neuspjeha. **/
    nextAttemptAt: Date
    /** Je li ovo događaj stvaranja, brisanja ili ažuriranja... **/
    eventType: OutboundSyncEventType
    /** Vrsta sinkronizacije koju treba izvršiti (WordPress, poziv API-ja, itd.). **/
    type: OutboundSyncType
    /** Domena koja odgovara komentaru. Tu domenu koristimo za odabir API ključa. **/
    domain: string
    /** Zadnja pogreška koja se dogodila. Ovaj tip nije tipiziran i predstavlja "dump" onoga što se dogodilo. Obično sadrži objekt sa statusCode, body i mapom headers. **/
    lastError: object | null
}
[inline-code-end]

---