Een `Notification` object vertegenwoordigt een notificatie voor een gebruiker.

`Notification` objects worden automatisch aangemaakt en kunnen niet via de API worden aangemaakt. Ze verlopen ook na één jaar.
Notificaties kunnen niet worden verwijderd. Ze kunnen echter wel worden bijgewerkt om `viewed` op `false` te zetten, en je kunt zoeken op `viewed`.

Een gebruiker kan zich ook afmelden voor notificaties voor een specifiek comment door `optedOut` in de notificatie op `true` te zetten. Je kunt je weer aanmelden door het op `false` te zetten.

Er zijn verschillende notificatie-types - controleer `relatedObjectType` en `type`.

De manieren waarop notificaties worden aangemaakt zijn vrij flexibel en kunnen door veel scenario's worden getriggerd (zie `NotificationType`).

Op dit moment impliceert het bestaan van een `Notification` niet noodzakelijk dat er een e-mail is of zou moeten worden verzonden. In plaats daarvan worden de notificaties gebruikt voor de notificatiefeed en gerelateerde integraties.

De structuur voor het `Notification` object is als volgt:

[inline-code-attrs-start title = 'Notification-structuur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
enum NotificationObjectType {
    Comment = 0,
    Profile = 1,
    Tenant = 2
}

enum NotificationType {
    /** Als iemand op jou heeft geantwoord. **/
    RepliedToMe = 0,
    /** Als iemand ergens in een thread heeft geantwoord (zelfs kinderen van kinderen) van een thread waarop je hebt gereageerd. **/
    RepliedTransientChild = 1,
    /** Als je comment werd omhoog gestemd. **/
    VotedMyComment = 2,
    /** Als er een nieuw comment wordt achtergelaten op de root van een pagina waarop je bent geabonneerd. **/
    SubscriptionReplyRoot = 3,
    /** Als iemand een reactie op je profiel heeft geplaatst. **/
    CommentedOnProfile = 4,
    /** Als je een privébericht hebt. **/
    DirectMessage = 5,
    /** TrialLimits is alleen voor tenant-gebruikers. **/
    TrialLimits = 6,
    /** Als je met @ genoemd bent. **/
    Mentioned = 7
}

interface Notification {
    id: string
    tenantId: string
    /** Bij SSO heeft de user id het formaat `<tenant id>:<user id>`. **/
    userId?: string
    /** Bij gebruik van SSO hoef je je alleen zorgen te maken over `userId`. **/
    anonUserId?: string
    /** urlId is bijna altijd gedefinieerd. Het is alleen optioneel voor notificaties op tenantniveau, die zeldzaam zijn. **/
    urlId?: string
    /** URL wordt gecached voor snelle navigatie naar de bron van de notificatie. **/
    url?: string
    /** Paginatitel wordt gecached voor snelle leesbaarheid van de notificatiebron. **/
    pageTitle?: string
    relatedObjectType: NotificationObjectType
    /** Bijvoorbeeld, comment id. **/
    relatedObjectId: string
    viewed: boolean
    createdAt: string // datumstring
    type: NotificationType
    fromCommentId?: string
    fromVoteId?: string
    /** fromUserName en fromUserAvatarSrc worden hier gecached voor snelle weergave van de notificatie. Ze worden bijgewerkt wanneer de gebruiker wordt bijgewerkt. **/
    fromUserName: string
    fromUserId: string
    fromUserAvatarSrc?: string
    /** Zet dit op true om te stoppen met het ontvangen van notificaties voor dit object. **/
    optedOut?: boolean
}
[inline-code-end]