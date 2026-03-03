FastComments provides an easy to use SSO solution. Updating a user's information with the HMAC-based integration is
as simple as having the user load the page with an updated payload.

However, it may be desirable to manage a user outside that flow, to improve consistency of your application.

The SSO User API provides a way to CRUD objects that we call SSOUsers. These objects are different from regular Users and
kept separate for type safety.

The structure for the SSOUser object is as follows:

[inline-code-attrs-start title = 'Struttura SSOUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUser {
    id: string
    username: string
    email?: string
    websiteUrl?: string
    signUpDate: number
    createdFromUrlId?: string
    loginCount?: number
    avatarSrc?: string
    optedInNotifications?: boolean
    optedInSubscriptionNotifications?: boolean
    displayLabel?: string
    displayName?: string
    isAccountOwner?: boolean // Permesso amministratore - gli utenti SSO con questa flag vengono fatturati come Admin SSO (separati dagli utenti SSO regolari)
    isAdminAdmin?: boolean // Permesso amministratore - gli utenti SSO con questa flag vengono fatturati come Admin SSO (separati dagli utenti SSO regolari)
    isCommentModeratorAdmin?: boolean // Permesso moderatore - gli utenti SSO con questa flag vengono fatturati come Moderatori SSO (separati dagli utenti SSO regolari)
    /** Se null, il Controllo Accessi non verrà applicato all'utente. Se una lista vuota, questo utente non potrà vedere alcuna pagina né menzionare altri utenti con @. **/
    groupIds?: string[] | null
    createdFromSimpleSSO?: boolean
    /** Non permettere ad altri utenti di vedere l'attività di questo utente, inclusi i commenti, nel suo profilo. Il valore predefinito è true per fornire profili sicuri. **/
    isProfileActivityPrivate?: boolean
    /** Non permettere ad altri utenti di lasciare commenti sul profilo di questo utente, né di vedere commenti di profilo esistenti. Predefinito false. **/
    isProfileCommentsPrivate?: boolean
    /** Non permettere ad altri utenti di inviare messaggi diretti a questo utente. Predefinito false. **/
    isProfileDMDisabled?: boolean
    karma?: number
    /** Configurazione opzionale per i badge utente. **/
    badgeConfig?: {
        /** Array di ID dei badge da assegnare all'utente. Limitato a 30 badge. L'ordine è rispettato. Si tratta di badge globali visibili in tutte le pagine. **/
        badgeIds: string[]
        /** Array di ID dei badge limitati alla pagina corrente (urlId). Questi badge sono mostrati solo sulla pagina in cui sono stati assegnati. **/
        pageBadgeIds?: string[]
        /** Se true, sostituisce tutti i badge attualmente mostrati con quelli forniti. I badge globali e quelli a livello di pagina sono sovrascritti indipendentemente. Se false, aggiunge ai badge esistenti. **/
        override?: boolean
        /** Se true, aggiorna le proprietà di visualizzazione dei badge dalla configurazione del tenant. **/
        update?: boolean
    }
}
[inline-code-end]

### Billing for SSO Users

SSO users are billed differently based on their permission flags:

- **Regular SSO Users**: Users without admin or moderator permissions are billed as regular SSO users
- **SSO Admins**: Users with `isAccountOwner` or `isAdminAdmin` flags are billed separately as SSO Admins (same rate as regular tenant admins)
- **SSO Moderators**: Users with `isCommentModeratorAdmin` flag are billed separately as SSO Moderators (same rate as regular moderators)

**Important**: To prevent double billing, the system automatically deduplicates SSO users against regular tenant users and moderators by email address. If an SSO user has the same email as a regular tenant user or moderator, they will not be billed twice.

### Access Control

Users can be broken into groups. This is what the `groupIds` field is for, and is optional.

### @Menzioni

By default `@mentions` will use `username` to search for other sso users when the `@` character is typed. If `displayName` is used, then results matching
`username` will be ignored when there is a match for `displayName`, and the `@mention` search results will use `displayName`.

### Sottoscrizioni

With FastComments, users can subscribe to a page by clicking the bell icon in the comment widget and clicking Subscribe.

With a regular user, we send them notification emails based on their notification settings.

With SSO Users, we split this up for backwards compatibility. Users will only get sent these additional subscription notification
emails if you set `optedInSubscriptionNotifications` to `true`.

### Badge

You can assign badges to SSO users using the `badgeConfig` property. Badges are visual indicators that appear next to a user's name in comments.

- `badgeIds` - An array of badge IDs to assign to the user. These are global badges visible on all pages. Must be valid badge IDs created in your FastComments account. Limited to 30 badges.
- `pageBadgeIds` - An optional array of badge IDs scoped to the current page (`urlId`). These badges are only displayed on the page where they were assigned. Different pages can have different page-scoped badges for the same user.
- `override` - If true, all existing displayed badges will be replaced with the provided ones. Global and page-scoped badges are overridden independently — overriding global badges does not affect page-scoped badges, and vice versa. If false or omitted, the provided badges will be added to any existing badges.
- `update` - If true, badge display properties will be updated from the tenant configuration whenever the user logs in.

---