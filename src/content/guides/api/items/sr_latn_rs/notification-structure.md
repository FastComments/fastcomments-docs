A `Notification` objekat predstavlja notifikaciju za korisnika.

`Notification` objekti se kreiraju automatski i ne mogu se kreirati preko API-ja. Takođe ističu nakon jedne godine.
Notifikacije se ne mogu obrisati. Međutim, mogu se ažurirati da se `viewed` postavi na `false`, i možete ih pretraživati po `viewed`.

Korisnik se takođe može odjaviti od notifikacija za određeni komentar postavljanjem `optedOut` u notifikaciji na `true`. Možete se ponovo prijaviti postavljanjem na `false`.

Postoje različiti tipovi notifikacija - proverite `relatedObjectType` i `type`.

Načini na koje se notifikacije kreiraju su prilično fleksibilni i mogu biti pokrenuti u mnogim scenarijima (videti `NotificationType`).

U ovom trenutku, postojanje `Notification` zapravo ne znači da je email poslat ili da bi trebao biti poslat. Umesto toga, notifikacije
se koriste za feed notifikacija i povezane integracije.

The structure for the `Notification` object is as follows:

[inline-code-attrs-start title = 'Struktura objekta Notification'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
enum NotificationObjectType {
    Comment = 0,
    Profile = 1,
    Tenant = 2
}

enum NotificationType {
    /** Ako vam je neko odgovorio. **/
    RepliedToMe = 0,
    /** Ako je neko odgovorio bilo gde u niti (čak i u potomcima potomaka) na kojoj ste komentarisali. **/
    RepliedTransientChild = 1,
    /** Ako je neko glasao za vaš komentar. **/
    VotedMyComment = 2,
    /** Ako je ostavljen novi komentar na korenu stranice na koju ste pretplaćeni. **/
    SubscriptionReplyRoot = 3,
    /** Ako je neko komentarisao vaš profil. **/
    CommentedOnProfile = 4,
    /** Ako imate privatnu poruku. **/
    DirectMessage = 5,
    /** TrialLimits je samo za korisnike tenanta. **/
    TrialLimits = 6,
    /** Ako ste bili pomenuti. **/
    Mentioned = 7
}

interface Notification {
    id: string
    tenantId: string
    /** Sa SSO, user id je u formatu `<tenant id>:<user id>`. **/
    userId?: string
    /** Kada radite sa SSO, treba da brinete samo o `userId`. **/
    anonUserId?: string
    /** urlId je skoro uvek definisan. Opcionalan je samo za notifikacije na nivou tenanta, koje su retke. **/
    urlId?: string
    /** URL je keširan za brzu navigaciju do izvora notifikacije. **/
    url?: string
    /** Naslov stranice je keširan za brzo čitanje izvora notifikacije. **/
    pageTitle?: string
    relatedObjectType: NotificationObjectType
    /** Na primer, id komentara. **/
    relatedObjectId: string
    viewed: boolean
    createdAt: string // date string
    type: NotificationType
    fromCommentId?: string
    fromVoteId?: string
    /** fromUserName i fromUserAvatarSrc su ovde keširani za brzo prikazivanje notifikacije. Ažuriraju se kada se korisnik ažurira. **/
    fromUserName: string
    fromUserId: string
    fromUserAvatarSrc?: string
    /** Postavite ovo na true da biste prestali da dobijate notifikacije za ovaj objekat. **/
    optedOut?: boolean
}
[inline-code-end]