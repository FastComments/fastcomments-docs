Objekat `Notification` predstavlja notifikaciju za korisnika.

Objekti `Notification` se kreiraju automatski i ne mogu se kreirati putem API-ja. Takođe ističu nakon godinu dana.
Notifikacije se ne mogu brisati. Međutim, mogu se ažurirati tako da se `viewed` postavi na `false`, i možete pretraživati po `viewed`.

Korisnik takođe može odustati od notifikacija za određeni komentar postavljanjem `optedOut` u notifikaciji na `true`. Možete se ponovo uključiti postavljanjem na `false`.

Postoje različiti tipovi notifikacija - pogledajte `relatedObjectType` i `type`.

Načini na koje se notifikacije kreiraju su prilično fleksibilni i mogu se aktivirati u mnogim scenarijima (vidite `NotificationType`).

Do danas, postojanje `Notification` zapravo ne podrazumeva da je email poslat ili da bi trebalo da bude poslat. Umesto toga, notifikacije
se koriste za feed notifikacija i povezane integracije.

Struktura objekta `Notification` je sledeća:

[inline-code-attrs-start title = 'Struktura Notification'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
enum NotificationObjectType {
    Comment = 0,
    Profile = 1,
    Tenant = 2
}

enum NotificationType {
    /** Ako vam je neko odgovorio. **/
    RepliedToMe = 0,
    /** Ako je neko odgovorio bilo gde u nizu (čak i deci dece) niza u kom ste komentarisali. **/
    RepliedTransientChild = 1,
    /** Ako je vaš komentar dobio plus. **/
    VotedMyComment = 2,
    /** Ako je novi komentar ostavljen na korenu stranice na koju ste pretplaćeni. **/
    SubscriptionReplyRoot = 3,
    /** Ako je neko komentarisao na vašem profilu. **/
    CommentedOnProfile = 4,
    /** Ako imate direktnu poruku. **/
    DirectMessage = 5,
    /** TrialLimits je samo za korisnike tenanta. **/
    TrialLimits = 6,
    /** Ako ste bili @pomenuti. **/
    Mentioned = 7
}

interface Notification {
    id: string
    tenantId: string
    /** Sa SSO, id korisnika je u formatu `<tenant id>:<user id>`. **/
    userId?: string
    /** Kada radite sa SSO, morate voditi računa samo o `userId`. **/
    anonUserId?: string
    /** urlId je skoro uvek definisan. Opciono je samo za notifikacije na nivou tenanta, koje su retke. **/
    urlId?: string
    /** URL se kešira za brzu navigaciju do izvora notifikacije. **/
    url?: string
    /** Naslov stranice se kešira za brzo čitanje izvora notifikacije. **/
    pageTitle?: string
    relatedObjectType: NotificationObjectType
    /** Na primer, id komentara. **/
    relatedObjectId: string
    viewed: boolean
    createdAt: string // date string
    type: NotificationType
    fromCommentId?: string
    fromVoteId?: string
    /** fromUserName i fromUserAvatarSrc su keširani ovde za brzo prikazivanje notifikacije. Ažuriraju se kada se korisnik ažurira. **/
    fromUserName: string
    fromUserId: string
    fromUserAvatarSrc?: string
    /** Postavite ovo na true da biste prestali da dobijate notifikacije za ovaj objekat. **/
    optedOut?: boolean
}
[inline-code-end]