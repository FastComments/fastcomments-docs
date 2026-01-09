---
Objekat `Notification` predstavlja obavještenje za korisnika.

`Notification` objekti se stvaraju automatski i ne mogu se kreirati putem API-ja. Također ističu nakon jedne godine.
Obavještenja ne mogu biti izbrisana. Međutim, mogu se ažurirati da bi se `viewed` postavio na `false`, i možete ih pretraživati po `viewed`.

Korisnik se također može odjaviti od obavještenja za određeni komentar tako što će u obavještenju postaviti `optedOut` na `true`. Možete se ponovo prijaviti postavljanjem na `false`.

Postoje različite vrste obavještenja - provjerite `relatedObjectType` i `type`.

Načini na koje se obavještenja kreiraju su prilično fleksibilni i mogu biti pokrenuti u mnogim scenarijima (pogledajte `NotificationType`).

Trenutno, postojanje `Notification` zapravo ne implicira da je e-pošta poslana ili da bi trebala biti poslana. Umjesto toga, obavještenja
se koriste za feed obavještenja i povezane integracije.

Struktura objekta `Notification` izgleda ovako:

[inline-code-attrs-start title = 'Struktura obavještenja'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
enum NotificationObjectType {
    Comment = 0,
    Profile = 1,
    Tenant = 2
}

enum NotificationType {
    /** Ako vam je neko odgovorio. **/
    RepliedToMe = 0,
    /** Ako je neko odgovorio bilo gdje u niti (čak i potomcima potomaka) niti u kojoj ste komentarisali. **/
    RepliedTransientChild = 1,
    /** Ako je vaš komentar dobio glas. **/
    VotedMyComment = 2,
    /** Ako je ostavljen novi komentar na korijenu stranice na koju ste pretplaćeni. **/
    SubscriptionReplyRoot = 3,
    /** Ako je neko komentarisao vaš profil. **/
    CommentedOnProfile = 4,
    /** Ako imate direktnu poruku. **/
    DirectMessage = 5,
    /** TrialLimits je samo za korisnike tenanta. **/
    TrialLimits = 6,
    /** Ako ste bili @spomenuti. **/
    Mentioned = 7
}

interface Notification {
    id: string
    tenantId: string
    /** Sa SSO, user id je u formatu `<tenant id>:<user id>`. **/
    userId?: string
    /** Kada radite sa SSO, morate se brinuti samo o `userId`. **/
    anonUserId?: string
    /** urlId je skoro uvijek definisan. Opcionalan je samo za notifikacije na nivou tenanta, koje su rijetke. **/
    urlId?: string
    /** URL je keširan za brzu navigaciju do izvora obavještenja. **/
    url?: string
    /** Naslov stranice je keširan za brzo čitanje izvora obavještenja. **/
    pageTitle?: string
    relatedObjectType: NotificationObjectType
    /** Na primjer, id komentara. **/
    relatedObjectId: string
    viewed: boolean
    createdAt: string // string datuma
    type: NotificationType
    fromCommentId?: string
    fromVoteId?: string
    /** fromUserName i fromUserAvatarSrc su keširani ovdje za brzo prikazivanje obavještenja. Ažuriraju se kada je korisnik ažuriran. **/
    fromUserName: string
    fromUserId: string
    fromUserAvatarSrc?: string
    /** Postavite ovo na true da prestanete primati obavještenja za ovaj objekt. **/
    optedOut?: boolean
}
[inline-code-end]

---