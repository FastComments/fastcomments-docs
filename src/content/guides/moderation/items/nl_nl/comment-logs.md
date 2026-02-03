FastComments houdt automatisch gedetailleerde gebeurtenissen bij voor elke opmerking om transparantie te bieden in moderatiebeslissingen en systeemacties. Deze logboeken helpen je te begrijpen waarom een opmerking werd goedgekeurd, als spam werd gemarkeerd of waarom de status is gewijzigd.

## Toegang tot de logboeken van opmerkingen

Om de logboeken voor een specifieke opmerking te bekijken:

1. Ga naar de **Moderate Comments**-pagina in je FastComments-dashboard
2. Zoek de opmerking die je wilt onderzoeken
3. Klik op de knop **View Logs** (klokpictogram) in de actiebalk van de opmerking
4. Er verschijnt een dialoogvenster met de volledige geschiedenis van gebeurtenissen voor die opmerking

Elke logvermelding toont:
- **When** - De timestamp van de gebeurtenis
- **Who** - De gebruiker of het systeem dat de gebeurtenis heeft veroorzaakt (indien van toepassing)
- **What** - Het type actie of gebeurtenis
- **Details** - Extra context zoals voor/na waarden, namen van engines of gerelateerde gegevens

## Evenementen in het commentlogboek

Elke opmerking behoudt een logboek van gebeurtenissen die tijdens de levenscyclus optreden. Hieronder staan de typen gebeurtenissen die worden bijgehouden:

### Anonymization Events
- **Anonymized** - De inhoud van de opmerking is gewist en de gebruiker is gemarkeerd als verwijderd
- **RestoredFromAnonymized** - Opmerking is hersteld vanuit een geanonimiseerde toestand

### Approval Events
- **ApprovedDueToPastComment** - Opmerking goedgekeurd omdat de gebruiker eerder goedgekeurde opmerkingen heeft (inclusief verwijzing naar de vorige opmerking)
- **ApprovedIsAdmin** - Opmerking goedgekeurd omdat de gebruiker een beheerder is
- **NotApprovedRequiresApproval** - Opmerking vereist handmatige goedkeuring
- **NotApprovedLowTrustFactor** - Opmerking niet goedgekeurd vanwege een lage gebruikersvertrouwensfactor (inclusief de trust factor waarde)

### Profile Comment Approval Events

Deze gebeurtenissen zijn specifiek van toepassing op opmerkingen op gebruikersprofielen:

- **ApprovedProfileAutoApproveAll** - Profielopmerking automatisch goedgekeurd omdat de profieleigenaar automatische goedkeuring voor alle opmerkingen heeft ingeschakeld
- **ApprovedProfileTrusted** - Profielopmerking goedgekeurd omdat de reageerder vertrouwd is (inclusief verwijzing naar de opmerking die vertrouwen heeft gevestigd)
- **NotApprovedProfileManualApproveAll** - Profielopmerking vereist handmatige goedkeuring omdat de profieleigenaar handmatige goedkeuring heeft ingeschakeld
- **NotApprovedProfileNotTrusted** - Profielopmerking niet goedgekeurd omdat de reageerder niet vertrouwd is
- **NotApprovedProfileNewUser** - Profielopmerking niet goedgekeurd omdat de reageerder een nieuwe gebruiker is

### Spam Detection Events
- **IsSpam** - Opmerking gemarkeerd als spam door de detectie-engine (inclusief welke engine de beslissing maakte)
- **IsSpamDueToBadWords** - Opmerking gemarkeerd als spam vanwege de scheldwoordenfilter
- **IsSpamFromLLM** - Opmerking gemarkeerd als spam door AI/LLM-engine (inclusief engine-naam, reactie en tokenaantal)
- **IsSpamRepeatComment** - Opmerking gemarkeerd als spam vanwege repetitie (inclusief welke engine dit detecteerde)
- **NotSpamIsOnlyImage** - Opmerking niet als spam gemarkeerd omdat deze alleen afbeeldingen bevat
- **NotSpamIsOnlyReacts** - Opmerking niet als spam gemarkeerd omdat deze alleen reacties bevat
- **NotSpamNoLinkOrMention** - Opmerking niet als spam gemarkeerd vanwege geen verdachte links of vermeldingen
- **NotSpamPerfectTrustFactor** - Opmerking niet als spam gemarkeerd vanwege hoge gebruikersvertrouwensfactor
- **NotSpamTooShort** - Opmerking niet als spam gemarkeerd omdat deze te kort is om te analyseren
- **NotSpamSkipped** - Spamcheck is overgeslagen
- **NotSpamFromEngine** - Opmerking bepaald als geen spam door detectie-engine (inclusief engine-naam en trust factor)

### Bad Words/Profanity Events
- **BadWordsCheckFailed** - Controle van de scheldwoordenfilter is mislukt
- **BadWordsFoundBadPhrase** - Scheldwoordenfilter heeft een ongepaste frase gedetecteerd (inclusief de frase)
- **BadWordsFoundBadWord** - Scheldwoordenfilter heeft een ongepast woord gedetecteerd (inclusief het woord)
- **BadWordsNoDefinitionForLocale** - Geen definities voor scheldwoorden beschikbaar voor de taal van de opmerking (inclusief de locale)

### User Verification Events
- **CommentMustBeVerifiedToApproveNotInVerifiedSession** - Opmerking vereist verificatie maar gebruiker is niet in een geverifieerde sessie
- **CommentMustBeVerifiedToApproveNotVerifiedYet** - Opmerking vereist verificatie maar gebruiker is nog niet geverifieerd
- **InVerifiedSession** - Gebruiker die de opmerking plaatst, bevindt zich in een geverifieerde sessie
- **SentVerificationEmailNoSession** - Verificatie-e-mail verzonden naar een niet-geverifieerde gebruiker
- **SentWelcomeEmail** - Welkomstmail verzonden naar nieuwe gebruiker

### Trust and Security Events
- **TrustFactorChanged** - De vertrouwensfactor van de gebruiker is gewijzigd (inclusief voor- en nawaarden)
- **SpamFilterDisabledBecauseAdmin** - Spamfiltering omzeild voor een beheerdersgebruiker
- **TenantSpamFilterDisabled** - Spamfiltering uitgeschakeld voor de hele tenant
- **RepeatCommentCheckIgnored** - Controle op herhaalde opmerkingen is overgeslagen (inclusief de reden)
- **UserIsAdmin** - Gebruiker geïdentificeerd als beheerder
- **UserIsAdminParentTenant** - Gebruiker geïdentificeerd als beheerder van de parent-tenant
- **UserIsAdminViaSSO** - Gebruiker geïdentificeerd als beheerder via SSO
- **UserIsMod** - Gebruiker geïdentificeerd als moderator

### Comment Status Changes

Statuswijzigingsgebeurtenissen bevatten voor- en nawaarden, plus de gebruiker die de wijziging heeft doorgevoerd:

- **ExpireStatusChanged** - De vervaldatumstatus van de opmerking is gewijzigd
- **ReviewStatusChanged** - De beoordelingsstatus van de opmerking is gewijzigd
- **SpamStatusChanged** - De spamstatus van de opmerking is bijgewerkt
- **ApproveStatusChanged** - De goedkeuringsstatus van de opmerking is gewijzigd
- **TextChanged** - De tekstinhoud van de opmerking is bewerkt (inclusief voor- en na-tekst)
- **VotesChanged** - Het aantal stemmen op de opmerking is bijgewerkt (inclusief gedetailleerde stemverdeling)
- **Flagged** - Opmerking is gemarkeerd door gebruikers
- **UnFlagged** - Markeringen van de opmerking zijn verwijderd

### Moderation Actions
- **Pinned** - Opmerking is vastgezet door een moderator (inclusief wie deze heeft vastgezet)
- **UnPinned** - Opmerking is losgemaakt door een moderator (inclusief wie deze heeft losgemaakt)

### Notification Events
- **CreatedNotifications** - Meldingen zijn aangemaakt voor de opmerking (inclusief het aantal meldingen)
- **NotificationCreateFailure** - Maken van meldingen is mislukt
- **BadgeAwarded** - Gebruikersbadge toegekend voor de opmerking (inclusief badge-naam)

### Publishing Events
- **PublishedLive** - Opmerking gepubliceerd naar live-abonnees (inclusief aantal abonnees)

### Integration Events
- **WebhookSynced** - Opmerking gesynchroniseerd via webhook

### Spam Rule Events
- **SpamRuleMatch** - Opmerking kwam overeen met een aangepaste spamregel (inclusief regeldetails)

### Localization Events
- **LocaleDetectedFromText** - Taal/locale automatisch gedetecteerd op basis van de opmerkingtekst (inclusief gedetecteerde taal en locale)

## Gebruiksscenario's voor commentlogboeken

Logboeken van opmerkingen worden automatisch gegenereerd en opgeslagen bij elke opmerking. Ze bieden waardevolle inzichten voor:

- **Inzicht in moderatiebeslissingen** - Zie precies waarom een opmerking werd goedgekeurd, ter beoordeling werd gehouden of als spam werd gemarkeerd
- **Foutopsporing van goedkeurings-/spamproblemen** - Volg de besluitvormingslogica wanneer opmerkingen zich niet gedragen zoals verwacht
- **Het volgen van gebruikersgedragspatronen** - Bewaak veranderingen in vertrouwensfactor en verificatiestatus
- **Auditen van moderatoracties** - Bekijk welke acties moderators op specifieke opmerkingen hebben uitgevoerd
- **Onderzoeken van de effectiviteit van spamfilters** - Zie welke detectie-engines spam vangen en welke niet
- **Probleemoplossing van integraties** - Verifieer webhook-synchronisaties en levering van meldingen

Deze logboeken helpen de transparantie in het moderatieproces te behouden en ondersteunen het verfijnen van het gedrag van je reactiesysteem.