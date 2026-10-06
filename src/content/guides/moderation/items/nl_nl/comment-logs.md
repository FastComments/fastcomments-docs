FastComments volgt automatisch gedetailleerde gebeurtenissen voor elke opmerking om transparantie te bieden in moderatiebeslissingen en systeemacties. Deze logboeken helpen je te begrijpen waarom een opmerking is goedgekeurd, gemarkeerd als spam, of waarvan de status is gewijzigd.

## Accessing Comment Logs

Om de logboeken voor een specifieke opmerking te bekijken:

1. Navigeer naar de **Moderate Comments** pagina in je FastComments-dashboard
2. Zoek de opmerking die je wilt inspecteren
3. Klik op de **View Logs** knop (klok‑icoon) in de actiebalk van de opmerking
4. Er verschijnt een dialoogvenster dat de volledige geschiedenis van gebeurtenissen voor die opmerking toont

Elke logboekvermelding toont:
- **When** - De tijdstempel van de gebeurtenis
- **Who** - De gebruiker of het systeem dat de gebeurtenis heeft geactiveerd (indien van toepassing)
- **What** - Het type actie of gebeurtenis
- **Details** - Extra context zoals voor/na waarden, engine‑namen, of gerelateerde gegevens

## Comment Log Events

Elke opmerking houdt een logboek bij van gebeurtenissen die zich tijdens de levenscyclus voordoen. Hieronder staan de soorten gebeurtenissen die worden bijgehouden:

### Anonymization Events
- **Anonymized** - Inhoud van de opmerking is gewist en gebruiker gemarkeerd als verwijderd
- **RestoredFromAnonymized** - Opmerking is hersteld vanuit geanonimiseerde status

### Approval Events
- **ApprovedDueToPastComment** - Opmerking goedgekeurd omdat de gebruiker eerder opmerkingen heeft goedgekeurd (bevat verwijzing naar de eerdere opmerking)
- **ApprovedIsAdmin** - Opmerking goedgekeurd omdat de gebruiker een beheerder is
- **NotApprovedRequiresApproval** - Opmerking vereist handmatige goedkeuring
- **NotApprovedLowTrustFactor** - Opmerking niet goedgekeurd vanwege een lage vertrouwensfactor van de gebruiker (bevat de vertrouwensfactorwaarde)

### Profile Comment Approval Events

These events apply specifically to comments on user profiles:
- **ApprovedProfileAutoApproveAll** - Profielopmerking automatisch goedgekeurd omdat de profiel eigenaar auto‑goedkeuring voor alle opmerkingen heeft ingeschakeld
- **ApprovedProfileTrusted** - Profielopmerking goedgekeurd omdat de reageerder vertrouwd is (bevat verwijzing naar de opmerking die vertrouwen heeft vastgesteld)
- **NotApprovedProfileManualApproveAll** - Profielopmerking vereist handmatige goedkeuring omdat de profiel eigenaar handmatige goedkeuring heeft ingeschakeld
- **NotApprovedProfileNotTrusted** - Profielopmerking niet goedgekeurd omdat de reageerder niet vertrouwd is
- **NotApprovedProfileNewUser** - Profielopmerking niet goedgekeurd omdat de reageerder een nieuwe gebruiker is

### Spam Detection Events
- **IsSpam** - Opmerking gemarkeerd als spam door detectie‑engine (bevat welke engine de beslissing heeft genomen)
- **IsSpamDueToBadWords** - Opmerking gemarkeerd als spam vanwege een scheldwoordfilter
- **IsSpamFromLLM** - Opmerking gemarkeerd als spam door AI/LLM‑engine (bevat engine‑naam, respons en token‑aantal)
- **IsSpamRepeatComment** - Opmerking gemarkeerd als spam omdat deze repetitief is (bevat welke engine het heeft gedetecteerd)
- **NotSpamIsOnlyImage** - Opmerking niet gemarkeerd als spam omdat deze alleen afbeeldingen bevat
- **NotSpamIsOnlyReacts** - Opmerking niet gemarkeerd als spam omdat deze alleen reacties bevat
- **NotSpamNoLinkOrMention** - Opmerking niet gemarkeerd als spam omdat er geen verdachte links of vermeldingen zijn
- **NotSpamPerfectTrustFactor** - Opmerking niet gemarkeerd als spam vanwege een hoge gebruikers‑vertrouwensfactor
- **NotSpamTooShort** - Opmerking niet gemarkeerd als spam omdat deze te kort is om te analyseren
- **NotSpamSkipped** - Spamcontrole werd overgeslagen
- **NotSpamFromEngine** - Opmerking bepaald als geen spam door detectie‑engine (bevat engine‑naam en vertrouwensfactor)

### Bad Words/Profanity Events
- **BadWordsCheckFailed** - Profanity‑filtercontrole liep op een fout
- **BadWordsFoundBadPhrase** - Profanity‑filter detecteerde ongepaste zin (bevat de zin)
- **BadWordsFoundBadWord** - Profanity‑filter detecteerde ongepast woord (bevat het woord)
- **BadWordsNoDefinitionForLocale** - Geen profanity‑definities beschikbaar voor de taal van de opmerking (bevat de locale)

### User Verification Events
- **CommentMustBeVerifiedToApproveNotInVerifiedSession** - Opmerking vereist verificatie maar gebruiker zit niet in een geverifieerde sessie
- **CommentMustBeVerifiedToApproveNotVerifiedYet** - Opmerking vereist verificatie maar gebruiker is nog niet geverifieerd
- **InVerifiedSession** - Gebruiker die de opmerking plaatst zit in een geverifieerde sessie
- **SentVerificationEmailNoSession** - Verificatie‑e‑mail verzonden naar niet‑geverifieerde gebruiker
- **SentWelcomeEmail** - Welkomst‑e‑mail verzonden naar nieuwe gebruiker

### Trust and Security Events
- **TrustFactorChanged** - Vertrouwensfactor van de gebruiker is gewijzigd (bevat voor‑ en na‑waarden)
- **SpamFilterDisabledBecauseAdmin** - Spamfilter omzeild voor beheerder
- **TenantSpamFilterDisabled** - Spamfilter uitgeschakeld voor de gehele tenant
- **RepeatCommentCheckIgnored** - Controle op herhaalde opmerkingen omzeild (bevat de reden)
- **UserIsAdmin** - Gebruiker geïdentificeerd als beheerder
- **UserIsAdminParentTenant** - Gebruiker geïdentificeerd als beheerder van de bovenliggende tenant
- **UserIsAdminViaSSO** - Gebruiker geïdentificeerd als beheerder via SSO
- **UserIsMod** - Gebruiker geïdentificeerd als moderator

### Comment Status Changes

Status change events include before and after values, plus the user who made the change:
- **ExpireStatusChanged** - Verloopstatus van de opmerking is gewijzigd
- **ReviewStatusChanged** - Review‑status van de opmerking is gewijzigd
- **SpamStatusChanged** - Spam‑status van de opmerking is bijgewerkt
- **ApproveStatusChanged** - Goedkeuringsstatus van de opmerking is gewijzigd
- **TextChanged** - Tekstinhoud van de opmerking is bewerkt (bevat voor‑ en na‑tekst)
- **VotesChanged** - Stemmen van de opmerking zijn bijgewerkt (bevat gedetailleerde stemverdeling)
- **Flagged** - Opmerking is gemarkeerd door gebruikers
- **UnFlagged** - Markeringen van de opmerking zijn verwijderd

### Moderation Actions
- **Pinned** - Opmerking is vastgezet door moderator (bevat wie het heeft vastgezet)
- **UnPinned** - Opmerking is losgemaakt door moderator (bevat wie het heeft losgemaakt)

### Notification Events
- **CreatedNotifications** - Meldingen zijn aangemaakt voor de opmerking (bevat aantal meldingen)
- **NotificationCreateFailure** - Mislukt om meldingen aan te maken
- **BadgeAwarded** - Gebruikersbadge is toegekend voor de opmerking (bevat badge‑naam)

### Mention and Reply Notification Events

These events name the person who would receive the email or notification. When nothing was sent, the Details column says why.
- **MentionEmailSent** - Een gebruiker die in de opmerking werd genoemd, kreeg een e‑mail
- **MentionEmailSkipped** - Een genoemde gebruiker kreeg geen e‑mail (bevat de reden)
- **MentionHeldForApproval** - De vermelding‑e‑mail wacht tot de opmerking is goedgekeurd
- **MentionNotificationCreated** - Een genoemde gebruiker kreeg een in‑app‑melding
- **MentionNotificationSkipped** - Een genoemde gebruiker kreeg geen in‑app‑melding (bevat de reden)
- **ReplyEmailSent** - De auteur van de beantwoordde opmerking kreeg een e‑mail over dit antwoord
- **ReplyEmailSkipped** - De auteur van de beantwoordde opmerking kreeg geen e‑mail (bevat de reden)
- **ReplyNotificationSkipped** - De auteur van de beantwoordde opmerking kreeg geen in‑app‑melding (bevat de reden)

Reasons shown when an email or notification was not sent:
- De gebruiker bestaat niet meer, of heeft geen e‑mailadres
- De gebruiker heeft e‑mailmeldingen uitgeschakeld, of meldingen voor die thread uitgeschakeld
- Een van de twee gebruikers heeft de ander geblokkeerd
- De gebruikers zitten niet in dezelfde SSO‑groepen
- Het e‑mailadres van de gebruiker staat op de onderdrukkingslijst na een bounce of spamklacht (zie [Email Suppression Management](/guide-notifications.html#email-suppression-management))
- Het e‑mailadres van de gebruiker is bij example.com, wat geen e‑mail kan ontvangen
- De opmerking is gemarkeerd als spam, verwijderd, of niet goedgekeurd binnen 7 dagen
- De beantwoordde opmerking is anoniem geplaatst
- De gebruiker heeft op zijn eigen opmerking geantwoord
- De gebruiker werd genoemd in het antwoord, dus kreeg hij de vermelding‑e‑mail in plaats van een antwoord‑e‑mail
- De gebruiker had al een antwoord‑melding voor de opmerking
- Verzenden is 5 keer mislukt

Als de levering faalt of een verzendlimiet bereikt, wordt de e‑mail in de wachtrij geplaatst om opnieuw te proberen en vermeldt het logboekitem dat.

### Publishing Events
- **PublishedLive** - Opmerking is gepubliceerd naar live abonnees (bevat aantal abonnees)

### Integration Events
- **WebhookSynced** - Opmerking is gesynchroniseerd via webhook

### Spam Rule Events
- **SpamRuleMatch** - Opmerking kwam overeen met een aangepaste spam‑regel (bevat regel‑details)

### Localization Events
- **LocaleDetectedFromText** - Taal‑locale werd automatisch gedetecteerd uit de opmerkingtekst (bevat gedetecteerde taal en locale)

## Use Cases for Comment Logs

Comment logs are automatically generated and stored with each comment. They provide valuable insights for:
- **Understanding moderation decisions** - Zie precies waarom een opmerking is goedgekeurd, in afwachting van beoordeling, of gemarkeerd als spam
- **Debugging approval/spam issues** - Volg de besluitvormingslogica wanneer opmerkingen zich niet gedragen zoals verwacht
- **Tracking user behavior patterns** - Houd veranderingen in vertrouwensfactor en verificatiestatus bij
- **Auditing moderator actions** - Bekijk welke acties moderators hebben ondernomen op specifieke opmerkingen
- **Investigating spam filter effectiveness** - Zie welke detectie‑engines spam oppikken en welke niet
- **Troubleshooting integrations** - Verifieer webhook‑synchronisaties en levering van meldingen

Deze logboeken helpen transparantie te behouden in het moderatieproces en ondersteunen bij het verfijnen van het gedrag van je commentaarsysteem.