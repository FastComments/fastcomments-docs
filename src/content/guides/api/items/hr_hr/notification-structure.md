Objekt `Notification` predstavlja obavijest za korisnika.

`Notification` objekti se stvaraju automatski i ne mogu se stvarati putem API-ja. Također istječu nakon jedne godine.
Obavijesti se ne mogu izbrisati. Međutim, mogu se ažurirati postavljanjem `viewed` na `false`, i možete ih pretraživati po `viewed`.

Korisnik se također može odjaviti od obavijesti za određeni komentar postavljanjem `optedOut` u obavijesti na `true`. Možete se ponovno prijaviti postavljanjem na `false`.

Postoje različite vrste obavijesti - provjerite `relatedObjectType` i `type`.

Načini na koje se obavijesti stvaraju su prilično fleksibilni i mogu biti pokrenuti u mnogim scenarijima (pogledajte `NotificationType`).

Trenutno, postojanje `Notification` ne znači nužno da je e-pošta poslana ili bi trebala biti poslana. Umjesto toga, obavijesti
se koriste za feed obavijesti i povezane integracije.

Struktura za `Notification` objekt izgleda ovako:

[inline-code-attrs-start title = 'Struktura objekta Notification'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
enum NotificationObjectType {
    Comment = 0,
    Profile = 1,
    Tenant = 2
}

enum NotificationType {
    /** Ako vam je netko odgovorio. **/
    RepliedToMe = 0,
    /** Ako je netko odgovorio negdje u niti (čak i u pod-nitima) niti na kojoj ste komentirali. **/
    RepliedTransientChild = 1,
    /** Ako je vaš komentar dobio glas. **/
    VotedMyComment = 2,
    /** Ako je ostavljen novi komentar na korijenu stranice na koju ste pretplaćeni. **/
    SubscriptionReplyRoot = 3,
    /** Ako je netko komentirao vaš profil. **/
    CommentedOnProfile = 4,
    /** Ako imate privatnu poruku (DM). **/
    DirectMessage = 5,
    /** TrialLimits je samo za korisnike tenant-a. **/
    TrialLimits = 6,
    /** Ako ste bili @mentioned. **/
    Mentioned = 7
}

interface Notification {
    id: string
    tenantId: string
    /** With SSO, the user id is in the format `<tenant id>:<user id>`. **/
    userId?: string
    /** When working with SSO, you only have to worry about `userId`. **/
    anonUserId?: string
    /** urlId is almost always defined. It is only optional for tenant-level notifications, which are infrequent. **/
    urlId?: string
    /** URL is cached for quick navigation to the source of the notification. **/
    url?: string
    /** Page Title is cached for quick reading of notification source. **/
    pageTitle?: string
    relatedObjectType: NotificationObjectType
    /** For example, comment id. **/
    relatedObjectId: string
    viewed: boolean
    createdAt: string // string datuma
    type: NotificationType
    fromCommentId?: string
    fromVoteId?: string
    /** fromUserName and fromUserAvatarSrc are cached here for quick displaying of the notification. They are updated when the user is updated. **/
    fromUserName: string
    fromUserId: string
    fromUserAvatarSrc?: string
    /** Set this to true to stop getting notifications for this object. **/
    optedOut?: boolean
}
[inline-code-end]

---