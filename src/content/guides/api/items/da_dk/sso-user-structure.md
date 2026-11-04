FastComments tilbyder en nem at bruge SSO-løsning. Opdatering af en brugers oplysninger med den HMAC-baserede integration er
lige så enkelt som at få brugeren til at indlæse siden med en opdateret payload.

Det kan dog være ønskeligt at administrere en bruger uden for dette flow, for at forbedre konsistensen i din applikation.

SSO User API giver en måde at udføre CRUD-operationer på objekter, som vi kalder SSOUsers. Disse objekter er forskellige fra almindelige Users og
holdes adskilt af hensyn til typesikkerhed.

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
    isAccountOwner?: boolean // Admin-tilladelse - SSO-brugere med dette flag faktureres som SSO Admins (adskilt fra almindelige SSO-brugere)
    isAdminAdmin?: boolean // Admin-tilladelse - SSO-brugere med dette flag faktureres som SSO Admins (adskilt fra almindelige SSO-brugere)
    isCommentModeratorAdmin?: boolean // Moderator-tilladelse - SSO-brugere med dette flag faktureres som SSO Moderatorer (adskilt fra almindelige SSO-brugere)
    /** Hvis null, anvendes Access Control ikke på brugeren. Hvis en tom liste, vil denne bruger ikke kunne se nogen sider eller @mention andre brugere. **/
    groupIds?: string[] | null
    createdFromSimpleSSO?: boolean
    Lad ikke andre brugere se denne brugers aktivitet, herunder kommentarer, på deres profil. Standard er true for som standard at give sikre profiler. **/
    isProfileActivityPrivate?: boolean
    /** Lad ikke andre brugere efterlade kommentarer på brugerens profil eller se eksisterende profilkommentarer. Standard false. **/
    isProfileCommentsPrivate?: boolean
    /** Lad ikke andre brugere sende direkte beskeder til denne bruger. Standard false. **/
    isProfileDMDisabled?: boolean
    karma?: number
    /** Valgfri konfiguration for brugerbadges. **/
    badgeConfig?: {
        /** Array af badge-ID'er, der skal tildeles brugeren. Begrænset til 30 badges. Rækkefølgen respekteres. Disse er globale badges, der er synlige på alle sider. **/
        badgeIds: string[]
        /** Array af badge-ID'er afgrænset til den aktuelle side (urlId). Disse badges vises kun på den side, hvor de blev tildelt. **/
        pageBadgeIds?: string[]
        /** Hvis true, erstattes alle eksisterende viste badges med de angivne. Globale og sideafgrænsede badges tilsidesættes uafhængigt. Hvis false, tilføjes til eksisterende badges. **/
        override?: boolean
        /** Hvis true, opdateres badge-visningsegenskaber fra tenant-konfigurationen. **/
        update?: boolean
    }
}
[inline-code-end]

### Fakturering af SSO-brugere

SSO-brugere faktureres forskelligt baseret på deres tilladelsesflag:

- **Almindelige SSO-brugere**: Brugere uden admin- eller moderator-tilladelser faktureres som almindelige SSO-brugere
- **SSO Admins**: Brugere med `isAccountOwner`- eller `isAdminAdmin`-flag faktureres separat som SSO Admins (samme sats som almindelige tenant-admins)
- **SSO Moderatorer**: Brugere med `isCommentModeratorAdmin`-flag faktureres separat som SSO Moderatorer (samme sats som almindelige moderatorer)

**Vigtigt**: For at forhindre dobbeltfakturering deduplikerer systemet automatisk SSO-brugere mod almindelige tenant-brugere og moderatorer via e-mailadresse. Hvis en SSO-bruger har samme e-mail som en almindelig tenant-bruger eller moderator, faktureres de ikke to gange.

### Access Control

Brugere kan opdeles i grupper. Det er dette, `groupIds`-feltet er til, og det er valgfrit.

### @Mentions

Som standard vil `@mentions` bruge `username` til at søge efter andre SSO-brugere, når `@`-tegnet tastes. Hvis `displayName` bruges, ignoreres resultater, der matcher
`username`, når der er et match for `displayName`, og `@mention`-søgeresultaterne vil bruge `displayName`.

### Abonnementer

Med FastComments kan brugere abonnere på en side ved at klikke på klokke-ikonet i kommentar-widgeten og klikke på Subscribe.

Med en almindelig bruger sender vi notifikations-e-mails baseret på deres notifikationsindstillinger.

Med SSO-brugere har vi opdelt dette af hensyn til bagudkompatibilitet. Brugere får kun tilsendt disse ekstra abonnementsnotifikations-e-mails,
hvis du sætter `optedInSubscriptionNotifications` til `true`.

### Badges

Du kan tildele badges til SSO-brugere via egenskaben `badgeConfig`. Badges er visuelle indikatorer, der vises ved siden af en brugers navn i kommentarer.

- `badgeIds` - Et array af badge-ID'er, der skal tildeles brugeren. Disse er globale badges, der er synlige på alle sider. Skal være gyldige badge-ID'er oprettet i din FastComments-konto. Begrænset til 30 badges.
- `pageBadgeIds` - Et valgfrit array af badge-ID'er afgrænset til den aktuelle side (`urlId`). Disse badges vises kun på den side, hvor de blev tildelt. Forskellige sider kan have forskellige sideafgrænsede badges for samme bruger.
- `override` - Hvis true, erstattes alle eksisterende viste badges med de angivne. Globale og sideafgrænsede badges tilsidesættes uafhængigt — tilsidesættelse af globale badges påvirker ikke sideafgrænsede badges, og omvendt. Hvis false eller udeladt, tilføjes de angivne badges til eventuelle eksisterende badges.
- `update` - Hvis true, opdateres badge-visningsegenskaber fra tenant-konfigurationen, hver gang brugeren logger ind.