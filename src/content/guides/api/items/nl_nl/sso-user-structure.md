FastComments biedt een gebruiksvriendelijke SSO-oplossing. Het bijwerken van de gegevens van een gebruiker met de HMAC-gebaseerde integratie is zo eenvoudig als de gebruiker de pagina laten laden met een bijgewerkte payload.

Het kan echter wenselijk zijn om een gebruiker buiten die flow te beheren, om de consistentie van uw applicatie te verbeteren.

De SSO User API biedt een manier om CRUD-bewerkingen uit te voeren op objecten die we SSOUsers noemen. Deze objecten verschillen van reguliere Users en worden apart gehouden voor typeveiligheid.

De structuur van het SSOUser-object is als volgt:

[inline-code-attrs-start title = 'SSOUser-structuur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
    isAccountOwner?: boolean // Admin-permissie - SSO-gebruikers met deze vlag worden gefactureerd als SSO Admins (apart van reguliere SSO-gebruikers)
    isAdminAdmin?: boolean // Admin-permissie - SSO-gebruikers met deze vlag worden gefactureerd als SSO Admins (apart van reguliere SSO-gebruikers)
    isCommentModeratorAdmin?: boolean // Moderator-permissie - SSO-gebruikers met deze vlag worden gefactureerd als SSO Moderators (apart van reguliere SSO-gebruikers)
    /** Als null is, wordt Toegangscontrole niet toegepast op de gebruiker. Als het een lege lijst is, kan deze gebruiker geen pagina's zien of andere gebruikers @mentionen. **/
    groupIds?: string[] | null
    createdFromSimpleSSO?: boolean
    /** Laat andere gebruikers de activiteit van deze gebruiker, inclusief reacties, niet zien op zijn profiel. Standaard is true om profielen standaard veilig te houden. **/
    isProfileActivityPrivate?: boolean
    /** Laat andere gebruikers geen opmerkingen achterlaten op het profiel van de gebruiker, of bestaande profielreacties zien. Standaard false. **/
    isProfileCommentsPrivate?: boolean
    /** Laat andere gebruikers geen privéberichten naar deze gebruiker sturen. Standaard false. **/
    isProfileDMDisabled?: boolean
    karma?: number
    /** Optionele configuratie voor gebruikersbadges. **/
    badgeConfig?: {
        /** Array met badge-ID's om aan de gebruiker toe te wijzen. Beperkt tot 30 badges. Volgorde wordt gerespecteerd. Dit zijn globale badges zichtbaar op alle pagina's. **/
        badgeIds: string[]
        /** Array met badge-ID's die zijn gescopeerd naar de huidige pagina (urlId). Deze badges worden alleen weergegeven op de pagina waar ze zijn toegewezen. **/
        pageBadgeIds?: string[]
        /** Als true, vervangt dit alle bestaande weergegeven badges door de opgegeven. Globale en pagina-gescopeerde badges worden onafhankelijk overschreven. Als false, worden ze toegevoegd aan bestaande badges. **/
        override?: boolean
        /** Als true, worden de weergave-eigenschappen van badges bijgewerkt vanuit de tenantconfiguratie. **/
        update?: boolean
    }
}
[inline-code-end]

### Facturering voor SSO-gebruikers

SSO-gebruikers worden verschillend gefactureerd op basis van hun permissievlaggen:

- **Regular SSO Users**: Gebruikers zonder admin- of moderatorrechten worden gefactureerd als reguliere SSO-gebruikers
- **SSO Admins**: Gebruikers met `isAccountOwner` of `isAdminAdmin` vlaggen worden apart gefactureerd als SSO Admins (zelfde tarief als reguliere tenant-beheerders)
- **SSO Moderators**: Gebruikers met de `isCommentModeratorAdmin` vlag worden apart gefactureerd als SSO Moderators (zelfde tarief als reguliere moderators)

**Belangrijk**: Om dubbele facturering te voorkomen, dedupliceert het systeem automatisch SSO-gebruikers ten opzichte van reguliere tenant-gebruikers en moderators op basis van e-mailadres. Als een SSO-gebruiker hetzelfde e-mailadres heeft als een reguliere tenant-gebruiker of moderator, wordt er niet twee keer gefactureerd.

### Toegangscontrole

Gebruikers kunnen in groepen worden verdeeld. Hiervoor is het veld `groupIds` bedoeld, en het is optioneel.

### @Mentions

Standaard zullen `@mentions` `username` gebruiken om andere sso-gebruikers te doorzoeken wanneer het `@` teken wordt getypt. Als `displayName` wordt gebruikt, worden resultaten die overeenkomen met `username` genegeerd wanneer er een overeenkomst is voor `displayName`, en zullen de `@mention`-zoekresultaten `displayName` gebruiken.

### Abonnementen

Met FastComments kunnen gebruikers zich op een pagina abonneren door op het belpictogram in de reactie-widget te klikken en op Abonneren te klikken.

Bij een reguliere gebruiker sturen we notificatie-e-mails op basis van hun notificatie-instellingen.

Bij SSO-gebruikers splitsen we dit om achterwaartse compatibiliteit te behouden. Gebruikers krijgen deze aanvullende abonnementsmeldings-e-mails alleen als u `optedInSubscriptionNotifications` op `true` zet.

### Badges

U kunt badges toewijzen aan SSO-gebruikers met behulp van de `badgeConfig`-eigenschap. Badges zijn visuele indicaties die naast de naam van een gebruiker in reacties verschijnen.

- `badgeIds` - Een array met badge-ID's om aan de gebruiker toe te wijzen. Dit zijn globale badges zichtbaar op alle pagina's. Moeten geldige badge-IDs zijn die in uw FastComments-account zijn gemaakt. Beperkt tot 30 badges.
- `pageBadgeIds` - Een optionele array met badge-ID's die zijn gescopeerd naar de huidige pagina (`urlId`). Deze badges worden alleen weergegeven op de pagina waar ze zijn toegewezen. Verschillende pagina's kunnen verschillende pagina-gescopeerde badges hebben voor dezelfde gebruiker.
- `override` - Als true, worden alle bestaande weergegeven badges vervangen door de opgegeven. Globale en pagina-gescopeerde badges worden onafhankelijk overschreven — het overschrijven van globale badges heeft geen effect op pagina-gescopeerde badges, en omgekeerd. Als false of weggelaten, worden de opgegeven badges toegevoegd aan eventuele bestaande badges.
- `update` - Als true, worden de weergave-eigenschappen van badges bij elke login van de gebruiker bijgewerkt vanuit de tenantconfiguratie.