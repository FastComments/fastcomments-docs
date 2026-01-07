FastComments giver en letanvendelig SSO-løsning. Opdatering af en brugers information med den HMAC-baserede integration er
så enkelt som at have brugeren indlæse siden med en opdateret payload.

Det kan dog være ønskeligt at administrere en bruger uden for det flow for at forbedre konsistensen af din applikation.

SSO User API'et giver en måde at CRUD objekter, som vi kalder SSOUsers. Disse objekter er forskellige fra almindelige Users og
holdes adskilt for typesikkerhed.

Strukturen for SSOUser-objektet er som følger:

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
    isAccountOwner?: boolean // Admin permission - SSO users with this flag are billed as SSO Admins (separate from regular SSO users)
    isAdminAdmin?: boolean // Admin permission - SSO users with this flag are billed as SSO Admins (separate from regular SSO users)
    isCommentModeratorAdmin?: boolean // Moderator permission - SSO users with this flag are billed as SSO Moderators (separate from regular SSO users)
    /** If null, Access Control will not be applied to the user. If an empty list, this user will not be able to see any pages or @mention other users. **/
    groupIds?: string[] | null
    createdFromSimpleSSO?: boolean
    /** Don't let other users see this user's activity, including comments, on their profile. Default is true to provide secure profiles by default. **/
    isProfileActivityPrivate?: boolean
    /** Don't let other users leave comments on the user's profile, or see existing profile comments. Default false. **/
    isProfileCommentsPrivate?: boolean
    /** Don't let other users send direct messages to this user. Default false. **/
    isProfileDMDisabled?: boolean
    karma?: number
    /** Optional configuration for user badges. **/
    badgeConfig?: {
        /** Array of badge IDs to assign to the user. Limited to 30 badges. Order is respected. **/
        badgeIds: string[]
        /** If true, replaces all existing displayed badges with the provided ones. If false, adds to existing badges. **/
        override?: boolean
        /** If true, updates badge display properties from tenant configuration. **/
        update?: boolean
    }
}
[inline-code-end]

### Fakturering for SSO-brugere

SSO-brugere faktureres forskelligt baseret på deres tilladelsesflags:

- **Almindelige SSO-brugere**: Brugere uden admin- eller moderatortilladelser faktureres som almindelige SSO-brugere
- **SSO-administratorer**: Brugere med `isAccountOwner` eller `isAdminAdmin` flags faktureres separat som SSO-administratorer (samme sats som almindelige tenant-administratorer)
- **SSO-moderatorer**: Brugere med `isCommentModeratorAdmin` flag faktureres separat som SSO-moderatorer (samme sats som almindelige moderatorer)

**Vigtigt**: For at forhindre dobbeltfakturering deduplikerer systemet automatisk SSO-brugere mod almindelige tenant-brugere og moderatorer efter e-mailadresse. Hvis en SSO-bruger har samme e-mail som en almindelig tenant-bruger eller moderator, vil de ikke blive faktureret to gange.

### Adgangskontrol

Brugere kan opdeles i grupper. Dette er hvad `groupIds`-feltet er til, og det er valgfrit.

### @Omtaler

Som standard vil `@mentions` bruge `username` til at søge efter andre sso-brugere, når `@`-tegnet skrives. Hvis `displayName` bruges, vil resultater, der matcher
`username`, blive ignoreret, når der er et match for `displayName`, og `@mention` søgeresultaterne vil bruge `displayName`.

### Abonnementer

Med FastComments kan brugere abonnere på en side ved at klikke på klokkeikonet i kommentar-widget'en og klikke Abonner.

Med en almindelig bruger sender vi dem notifikations-e-mails baseret på deres notifikationsindstillinger.

Med SSO-brugere opdeler vi dette af hensyn til bagudkompatibilitet. Brugere vil kun få sendt disse yderligere abonnementsnotifikations-
e-mails, hvis du sætter `optedInSubscriptionNotifications` til `true`.

### Badges

Du kan tildele badges til SSO-brugere ved hjælp af `badgeConfig`-egenskaben. Badges er visuelle indikatorer, der vises ved siden af en brugers navn i kommentarer.

- `badgeIds` - Et array af badge-ID'er til at tildele brugeren. Disse skal være gyldige badge-ID'er oprettet i din FastComments-konto. Begrænset til 30 badges.
- `override` - Hvis sand, vil alle eksisterende badges, der vises på kommentarer, blive erstattet med de angivne. Hvis falsk eller udeladt, vil de angivne badges blive tilføjet til eventuelle eksisterende badges.
- `update` - Hvis sand, vil badge-visningsegenskaber blive opdateret fra tenant-konfigurationen, når brugeren logger ind.
