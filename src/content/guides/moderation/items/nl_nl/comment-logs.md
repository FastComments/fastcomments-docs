FastComments registreert automatisch gedetailleerde gebeurtenissen voor elke reactie om transparantie te bieden bij moderatiebeslissingen en systeemacties. Deze logs helpen je te begrijpen waarom een reactie is goedgekeurd, als spam is gemarkeerd of waarom de status is gewijzigd.

Je kunt reactielogs voor individuele reacties bekijken in het Moderate Comments-dashboard door een specifieke reactie te selecteren.

## Gebeurtenissen in het reactielog

Elke reactie onderhoudt een log van gebeurtenissen die zich tijdens de levenscyclus voordoen. Hieronder staan de soorten gebeurtenissen die worden vastgelegd:

### Anonimiseringsgebeurtenissen
- **Anonymized** - Reactie-inhoud is gewist en gebruiker gemarkeerd als verwijderd

### Goedkeuringsgebeurtenissen
- **ApprovedDueToPastComment** - Reactie goedgekeurd omdat de gebruiker eerder goedgekeurde reacties heeft
- **ApprovedIsAdmin** - Reactie goedgekeurd omdat de gebruiker een beheerder is
- **NotApprovedRequiresApproval** - Reactie vereist handmatige goedkeuring

### Spamdetectiegebeurtenissen
- **IsSpam** - Reactie door detectiemotor als spam gemarkeerd
- **IsSpamDueToBadWords** - Reactie als spam gemarkeerd door vloekenfilter
- **IsSpamFromLLM** - Reactie door AI/LLM-engine als spam gemarkeerd
- **IsSpamRepeatComment** - Reactie als spam gemarkeerd vanwege herhaling
- **NotSpamIsOnlyImage** - Reactie niet als spam gemarkeerd omdat deze alleen afbeeldingen bevat
- **NotSpamIsOnlyReacts** - Reactie niet als spam gemarkeerd omdat deze alleen reacties bevat
- **NotSpamNoLinkOrMention** - Reactie niet als spam gemarkeerd vanwege geen verdachte links of vermeldingen
- **NotSpamPerfectTrustFactor** - Reactie niet als spam gemarkeerd vanwege hoge gebruikersvertrouwensscore
- **NotSpamTooShort** - Reactie niet als spam gemarkeerd omdat deze te kort is om te analyseren
- **NotSpamSkipped** - Spamcontrole is overgeslagen
- **NotSpamFromEngine** - Reactie door detectiemotor niet als spam beschouwd

### Gebeurtenissen met betrekking tot ongepaste woorden/vloeken
- **BadWordsCheckFailed** - Controle van het vloekenfilter is mislukt
- **BadWordsFoundBadPhrase** - Vloekenfilter heeft een ongepaste zin gedetecteerd
- **BadWordsFoundBadWord** - Vloekenfilter heeft een ongepast woord gedetecteerd
- **BadWordsNoDefinitionForLocale** - Geen definities voor ongepaste woorden beschikbaar voor de taal van de reactie

### Gebruikersverificatiegebeurtenissen
- **CommentMustBeVerifiedToApproveNotInVerifiedSession** - Reactie vereist verificatie maar gebruiker zit niet in een geverifieerde sessie
- **CommentMustBeVerifiedToApproveNotVerifiedYet** - Reactie vereist verificatie maar gebruiker is nog niet geverifieerd
- **InVerifiedSession** - Gebruiker die de reactie plaatst bevindt zich in een geverifieerde sessie
- **SentVerificationEmailNoSession** - Verificatie-e-mail verzonden naar niet-geverifieerde gebruiker
- **SentWelcomeEmail** - Welkomst-e-mail verzonden naar nieuwe gebruiker

### Vertrouwens- en beveiligingsgebeurtenissen
- **TrustFactorChanged** - Vertrouwensfactor van gebruiker is aangepast
- **SpamFilterDisabledBecauseAdmin** - Spamfiltering overgeslagen voor beheerder
- **TenantSpamFilterDisabled** - Spamfiltering uitgeschakeld voor gehele tenant
- **RepeatCommentCheckIgnored** - Controle op herhaalde reacties is overgeslagen
- **UserIsAdmin** - Gebruiker geïdentificeerd als beheerder
- **UserIsAdminParentTenant** - Gebruiker geïdentificeerd als beheerder van de parent-tenant
- **UserIsAdminViaSSO** - Gebruiker geïdentificeerd als beheerder via SSO
- **UserIsMod** - Gebruiker geïdentificeerd als moderator

### Wijzigingen in reactiestatus
- **ExpireStatusChanged** - Verloopstatus van de reactie is aangepast
- **ReviewStatusChanged** - Reviewstatus van de reactie is gewijzigd
- **SpamStatusChanged** - Spamstatus van de reactie is bijgewerkt
- **ApproveStatusChanged** - Goedkeuringsstatus van de reactie is gewijzigd
- **TextChanged** - Tekstinhoud van de reactie is bewerkt
- **VotesChanged** - Stemcijfers van de reactie zijn bijgewerkt
- **Flagged** - Reactie is door gebruikers gemarkeerd
- **UnFlagged** - Markeringen van de reactie zijn verwijderd

### Moderatieacties
- **Pinned** - Reactie is vastgepind door moderator
- **UnPinned** - Reactie is losgemaakt door moderator
- **RestoredFromAnonymized** - Reactie is hersteld vanuit geanonimiseerde staat

### Meldingsgebeurtenissen
- **CreatedNotifications** - Meldingen zijn aangemaakt voor de reactie
- **NotificationCreateFailure** - Maken van meldingen is mislukt
- **BadgeAwarded** - Gebruikersbadge toegekend voor reactie

### Publicatiegebeurtenissen
- **PublishedLive** - Reactie is gepubliceerd naar live-abonnees

### Integratiegebeurtenissen
- **WebhookSynced** - Reactie is gesynchroniseerd via webhook

### Spamregelgebeurtenissen
- **SpamRuleMatch** - Reactie voldeed aan een aangepaste spamregel

## Toegang tot reactielogboeken

Reactielogs worden automatisch gegenereerd en opgeslagen bij elke reactie. Ze bieden waardevolle inzichten voor:

- Begrijpen van moderatiebeslissingen
- Foutopsporing van goedkeurings-/spamproblemen
- Het volgen van gebruikersgedrags patronen
- Auditing van systeemacties

Deze logs helpen de transparantie in het moderatieproces te behouden en ondersteunen bij het verfijnen van het gedrag van je reactiesysteem.