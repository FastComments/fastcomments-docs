FastComments biedt een eenvoudig te gebruiken SSO-oplossing. Het bijwerken van de informatie van een gebruiker met de HMAC-gebaseerde integratie is zo eenvoudig als de gebruiker de pagina laten laden met een bijgewerkte payload.

Het kan echter wenselijk zijn om een gebruiker buiten dat proces te beheren, om de consistentie van uw applicatie te verbeteren.

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
    isAccountOwner?: boolean // Bevoegdheid voor beheerder - SSO-gebruikers met deze vlag worden gefactureerd als SSO-beheerders (apart van reguliere SSO-gebruikers)
    isAdminAdmin?: boolean // Bevoegdheid voor beheerder - SSO-gebruikers met deze vlag worden gefactureerd als SSO-beheerders (apart van reguliere SSO-gebruikers)
    isCommentModeratorAdmin?: boolean // Bevoegdheid voor moderator - SSO-gebruikers met deze vlag worden gefactureerd als SSO-moderators (apart van reguliere SSO-gebruikers)
    /** Als null wordt Access Control niet op de gebruiker toegepast. Als het een lege lijst is, kan deze gebruiker geen pagina's zien of andere gebruikers @mentionen. **/
    groupIds?: string[] | null
    createdFromSimpleSSO?: boolean
    /** Voorkom dat andere gebruikers de activiteit van deze gebruiker, inclusief reacties, op zijn/haar profiel zien. Standaard is true om standaard veilige profielen te bieden. **/
    isProfileActivityPrivate?: boolean
    /** Voorkom dat andere gebruikers reacties plaatsen op het profiel van de gebruiker, of bestaande profielreacties zien. Standaard false. **/
    isProfileCommentsPrivate?: boolean
    /** Voorkom dat andere gebruikers directe berichten naar deze gebruiker sturen. Standaard false. **/
    isProfileDMDisabled?: boolean
    karma?: number
    /** Optionele configuratie voor gebruikersbadges. **/
    badgeConfig?: {
        /** Array met badge-IDs om aan de gebruiker toe te wijzen. Beperkt tot 30 badges. Volgorde wordt gerespecteerd. **/
        badgeIds: string[]
        /** Als true vervangt dit alle bestaande weergegeven badges door de opgegeven. Als false, worden ze toegevoegd aan bestaande badges. **/
        override?: boolean
        /** Als true werkt dit de weergave-eigenschappen van badges bij vanuit de tenantconfiguratie. **/
        update?: boolean
    }
}
[inline-code-end]

### Facturering voor SSO-gebruikers

SSO-gebruikers worden anders gefactureerd afhankelijk van hun permissievlaggen:

- **Reguliere SSO-gebruikers**: Gebruikers zonder admin- of moderatorrechten worden gefactureerd als reguliere SSO-gebruikers
- **SSO-beheerders**: Gebruikers met de `isAccountOwner` of `isAdminAdmin` vlaggen worden apart gefactureerd als SSO-beheerders (zelfde tarief als reguliere tenantbeheerders)
- **SSO-moderators**: Gebruikers met de `isCommentModeratorAdmin` vlag worden apart gefactureerd als SSO-moderators (zelfde tarief als reguliere moderators)

**Belangrijk**: Om dubbele facturatie te voorkomen, dedupliceert het systeem automatisch SSO-gebruikers ten opzichte van reguliere tenantgebruikers en moderators op basis van e-mailadres. Als een SSO-gebruiker hetzelfde e-mailadres heeft als een reguliere tenantgebruiker of moderator, zullen ze niet twee keer gefactureerd worden.

### Toegangscontrole

Gebruikers kunnen in groepen worden ingedeeld. Hiervoor is het veld `groupIds` bedoeld, en het is optioneel.

### @Vermeldingen

Standaard zal `@mentions` `username` gebruiken om naar andere SSO-gebruikers te zoeken wanneer het `@`-teken wordt getypt. Als `displayName` wordt gebruikt, zullen resultaten die overeenkomen met `username` worden genegeerd wanneer er een match is voor `displayName`, en zullen de `@mention`-zoekresultaten `displayName` gebruiken.

### Abonnementen

Met FastComments kunnen gebruikers zich abonneren op een pagina door op het belpictogram in de commentaarwidget te klikken en op Abonneren te klikken.

Bij een reguliere gebruiker sturen we ze notificatie-e-mails op basis van hun notificatie-instellingen.

Bij SSO-gebruikers hebben we dit opgesplitst voor achterwaartse compatibiliteit. Gebruikers zullen deze extra abonnementsmeldingen alleen ontvangen als u `optedInSubscriptionNotifications` op `true` zet.

### Badges

U kunt badges toewijzen aan SSO-gebruikers met de eigenschap `badgeConfig`. Badges zijn visuele indicatoren die naast de naam van een gebruiker bij reacties verschijnen.

- `badgeIds` - Een array met badge-IDs om aan de gebruiker toe te wijzen. Deze moeten geldige badge-IDs zijn die in uw FastComments-account zijn aangemaakt. Beperkt tot 30 badges.
- `override` - Als true worden alle bestaande badges die bij reacties worden weergegeven vervangen door de opgegeven badges. Als false of weggelaten, worden de opgegeven badges toegevoegd aan eventuele bestaande badges.
- `update` - Als true worden de weergave-eigenschappen van badges bij elke login van de gebruiker bijgewerkt vanuit de tenantconfiguratie.

---