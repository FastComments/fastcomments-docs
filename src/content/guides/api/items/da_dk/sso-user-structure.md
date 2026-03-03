FastComments provides an easy to use SSO solution. Updating a user's information with the HMAC-based integration is
as simple as having the user load the page with an updated payload.

However, it may be desirable to manage a user outside that flow, to improve consistency of your application.

The SSO User API provides a way to CRUD objects that we call SSOUsers. These objects are different from regular Users and
kept separate for type safety.

The structure for the SSOUser object is as follows:

[inline-code-attrs-start title = 'SSOUser Struktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
    isAccountOwner?: boolean // Administratorrettighed - SSO users med dette flag faktureres som SSO Admins (separat fra regular SSO users)
    isAdminAdmin?: boolean // Administratorrettighed - SSO users med dette flag faktureres som SSO Admins (separat fra regular SSO users)
    isCommentModeratorAdmin?: boolean // Moderatorrettighed - SSO users med dette flag faktureres som SSO Moderators (separat fra regular SSO users)
    /** Hvis null, vil Access Control ikke blive anvendt på useren. Hvis en tom liste, vil denne user ikke kunne se nogen sider eller @mention andre users. **/
    groupIds?: string[] | null
    createdFromSimpleSSO?: boolean
    /** Gør det umuligt for andre users at se denne users aktivitet, inklusive kommentarer, på deres profil. Standard er true for at give sikre profiler som standard. **/
    isProfileActivityPrivate?: boolean
    /** Forhindrer andre users i at efterlade kommentarer på userens profil, eller se eksisterende profilkommentarer. Standard er false. **/
    isProfileCommentsPrivate?: boolean
    /** Forhindrer andre users i at sende direkte beskeder til denne user. Standard er false. **/
    isProfileDMDisabled?: boolean
    karma?: number
    /** Valgfri konfiguration for user badges. **/
    badgeConfig?: {
        /** Array af badge IDs der tildeles useren. Begrænset til 30 badges. Rækkefølgen respekteres. Dette er globale badges synlige på alle sider. **/
        badgeIds: string[]
        /** Array af badge IDs scoped til den aktuelle side (urlId). Disse badges vises kun på den side, hvor de blev tildelt. **/
        pageBadgeIds?: string[]
        /** Hvis true, erstattes alle eksisterende viste badges med de angivne. Globale og page-scoped badges overskrives uafhængigt. Hvis false, tilføjes til eksisterende badges. **/
        override?: boolean
        /** Hvis true opdateres badge-visningsindstillinger fra tenant-konfigurationen. **/
        update?: boolean
    }
}
[inline-code-end]

### Fakturering for SSO users

SSO users are billed differently based on their permission flags:

- **Regular SSO Users**: Users uden admin- eller moderatorrettigheder faktureres som regular SSO users
- **SSO Admins**: Users med `isAccountOwner` eller `isAdminAdmin` flags faktureres separat som SSO Admins (samme takst som regular tenant admins)
- **SSO Moderators**: Users med `isCommentModeratorAdmin` flag faktureres separat som SSO Moderators (samme takst som regular moderators)

**Important**: For at forhindre dobbeltfakturering deduplikerer systemet automatisk SSO users i forhold til regular tenant users og moderators ud fra e-mailadressen. Hvis en SSO user har samme email som en regular tenant user eller moderator, vil de ikke blive faktureret to gange.

### Adgangskontrol

Users kan opdeles i grupper. Det er det, `groupIds`-feltet er til, og det er valgfrit.

### @Mentions

Som standard bruger `@mentions` `username` til at søge efter andre sso users, når tegnet `@` tastes. Hvis `displayName` bruges, vil resultater, der matcher
`username`, blive ignoreret, når der er et match for `displayName`, og `@mention`-søgeresultaterne vil bruge `displayName`.

### Abonnementer

Med FastComments kan users abonnere på en side ved at klikke på klokkeikonet i kommentar-widgeten og klikke på Subscribe.

Med en regular user sender vi dem notifikations-e-mails baseret på deres notifikationsindstillinger.

Med SSO users deler vi dette op for bagudkompatibilitet. Users vil kun modtage disse ekstra abonnementsnotifikations-e-mails, hvis du sætter `optedInSubscriptionNotifications` til `true`.

### Badges

Du kan tildele badges til SSO users ved hjælp af `badgeConfig`-egenskaben. Badges er visuelle indikatorer, der vises ved siden af en users navn i kommentarer.

- `badgeIds` - Et array af badge IDs der tildeles useren. Disse er globale badges synlige på alle sider. Must be valid badge IDs created in your FastComments account. Limited to 30 badges.
- `pageBadgeIds` - Et valgfrit array af badge IDs scoped til den aktuelle side (`urlId`). Disse badges vises kun på den side, hvor de blev tildelt. Forskellige sider kan have forskellige page-scoped badges for samme user.
- `override` - Hvis true, vil alle eksisterende viste badges blive erstattet med de angivne. Globale og page-scoped badges overskrives uafhængigt — overskrivning af globale badges påvirker ikke page-scoped badges, og omvendt. Hvis false eller udeladt, tilføjes de angivne badges til eventuelle eksisterende badges.
- `update` - Hvis true, opdateres badge-visningsindstillinger fra tenant-konfigurationen, hver gang user logger ind.

---