FastComments registreert automatisch gedetailleerde gebeurtenissen voor elke opmerking om transparantie te bieden in moderatiebeslissingen en systeemacties. Deze logboeken helpen je te begrijpen waarom een opmerking is goedgekeurd, als spam gemarkeerd, of waarvan de status is gewijzigd.

## Toegang tot Opmerkinglogboeken

Om de logboeken van een specifieke opmerking te bekijken:

1. Ga naar de pagina **Moderate Comments** in je FastComments-dashboard
2. Zoek de opmerking die je wilt inspecteren
3. Klik op de knop **View Logs** (klok-icoon) in de actiebalk van de opmerking
4. Er verschijnt een dialoogvenster met de volledige geschiedenis van gebeurtenissen voor die opmerking

Elk logboekitem toont:
- **Wanneer** - Het tijdstip van de gebeurtenis
- **Wie** - De gebruiker of het systeem dat de gebeurtenis heeft veroorzaakt (indien van toepassing)
- **Wat** - Het type actie of gebeurtenis
- **Details** - Aanvullende context zoals voor/na-waarden, engine-namen of gerelateerde gegevens

## Opmerkinglogboek-gebeurtenissen

Elke opmerking bijhoudt een logboek van gebeurtenissen die plaatsvinden tijdens de levenscyclus. Hieronder staan de typen gebeurtenissen die worden bijgehouden:

### Anonimiseringsgebeurtenissen
- **Anonymized** - De inhoud van de opmerking is gewist en de gebruiker is gemarkeerd als verwijderd
- **RestoredFromAnonymized** - De opmerking is hersteld vanuit een geanonimiseerde staat

### Goedkeuringsgebeurtenissen
- **ApprovedDueToPastComment** - Opmerking goedgekeurd omdat de gebruiker eerder goedgekeurde opmerkingen heeft (bevat verwijzing naar de eerdere opmerking)
- **ApprovedIsAdmin** - Opmerking goedgekeurd omdat de gebruiker een admin is
- **NotApprovedRequiresApproval** - Opmerking vereist handmatige goedkeuring
- **NotApprovedLowTrustFactor** - Opmerking niet goedgekeurd vanwege een lage trustfactor van de gebruiker (bevat de trustfactor-waarde)

### Goedkeuringsgebeurtenissen voor profielopmerkingen

Deze gebeurtenissen gelden specifiek voor opmerkingen op gebruikersprofielen:

- **ApprovedProfileAutoApproveAll** - Profielopmerking automatisch goedgekeurd omdat de profieleigenaar auto-goedkeuring voor alle opmerkingen heeft ingeschakeld
- **ApprovedProfileTrusted** - Profielopmerking goedgekeurd omdat de reageerder vertrouwd wordt (bevat verwijzing naar de opmerking die het vertrouwen vestigde)
- **NotApprovedProfileManualApproveAll** - Profielopmerking vereist handmatige goedkeuring omdat de profieleigenaar handmatige goedkeuring heeft ingeschakeld
- **NotApprovedProfileNotTrusted** - Profielopmerking niet goedgekeurd omdat de reageerder niet vertrouwd wordt
- **NotApprovedProfileNewUser** - Profielopmerking niet goedgekeurd omdat de reageerder een nieuwe gebruiker is

### Spamdetectie-gebeurtenissen
- **IsSpam** - Opmerking gemarkeerd als spam door de detectie-engine (bevat welke engine de beslissing nam)
- **IsSpamDueToBadWords** - Opmerking gemarkeerd als spam vanwege het grof taalgebruik-filter
- **IsSpamFromLLM** - Opmerking gemarkeerd als spam door de AI/LLM-engine (bevat engine-naam, respons en tokenaantal)
- **IsSpamRepeatComment** - Opmerking gemarkeerd als spam omdat deze repetitief is (bevat welke engine dit detecteerde)
- **NotSpamIsOnlyImage** - Opmerking niet gemarkeerd als spam omdat deze alleen afbeeldingen bevat
- **NotSpamIsOnlyReacts** - Opmerking niet gemarkeerd als spam omdat deze alleen reacties bevat
- **NotSpamNoLinkOrMention** - Opmerking niet gemarkeerd als spam vanwege het ontbreken van verdachte links of vermeldingen
- **NotSpamPerfectTrustFactor** - Opmerking niet gemarkeerd als spam vanwege hoge gebruikersvertrouwen
- **NotSpamTooShort** - Opmerking niet gemarkeerd als spam omdat deze te kort is om te analyseren
- **NotSpamSkipped** - De spamcontrole is overgeslagen
- **NotSpamFromEngine** - Opmerking door de detectie-engine als geen spam bepaald (bevat engine-naam en trustfactor)

### Gebeurtenissen voor grof taalgebruik
- **BadWordsCheckFailed** - De controle van het grof taalgebruik-filter liep tegen een fout aan
- **BadWordsFoundBadPhrase** - Het grof taalgebruik-filter detecteerde een ongepaste zin (bevat de zin)
- **BadWordsFoundBadWord** - Het grof taalgebruik-filter detecteerde een ongepast woord (bevat het woord)
- **BadWordsNoDefinitionForLocale** - Geen definities voor grof taalgebruik beschikbaar voor de taal van de opmerking (bevat de locale)

### Gebeurtenissen voor gebruikersverificatie
- **CommentMustBeVerifiedToApproveNotInVerifiedSession** - Opmerking vereist verificatie maar de gebruiker is niet in een geverifieerde sessie
- **CommentMustBeVerifiedToApproveNotVerifiedYet** - Opmerking vereist verificatie maar de gebruiker is nog niet geverifieerd
- **InVerifiedSession** - De gebruiker die de opmerking plaatst, is in een geverifieerde sessie
- **SentVerificationEmailNoSession** - Verificatie-e-mail verzonden naar een niet-geverifieerde gebruiker
- **SentWelcomeEmail** - Welkomste-mail verzonden naar een nieuwe gebruiker

### Vertrouwens- en beveiligingsgebeurtenissen
- **TrustFactorChanged** - De trustfactor van de gebruiker is gewijzigd (bevat voor- en nawaarden)
- **SpamFilterDisabledBecauseAdmin** - Spamfiltering omzeild voor een admin-gebruiker
- **TenantSpamFilterDisabled** - Spamfiltering uitgeschakeld voor de hele tenant
- **RepeatCommentCheckIgnored** - De controle op herhaalde opmerkingen is omzeild (bevat de reden)
- **UserIsAdmin** - Gebruiker geïdentificeerd als admin
- **UserIsAdminParentTenant** - Gebruiker geïdentificeerd als admin van de parent tenant
- **UserIsAdminViaSSO** - Gebruiker geïdentificeerd als admin via SSO
- **UserIsMod** - Gebruiker geïdentificeerd als moderator

### Statuswijzigingen van opmerkingen

Gebeurtenissen voor statuswijzigingen bevatten voor- en nawaarden, plus de gebruiker die de wijziging heeft doorgevoerd:

- **ExpireStatusChanged** - De vervaldatumstatus van de opmerking is gewijzigd
- **ReviewStatusChanged** - De beoordelingsstatus van de opmerking is gewijzigd
- **SpamStatusChanged** - De spamstatus van de opmerking is bijgewerkt
- **ApproveStatusChanged** - De goedkeuringsstatus van de opmerking is gewijzigd
- **TextChanged** - De tekstinhoud van de opmerking is bewerkt (bevat voor- en natekst)
- **VotesChanged** - De stemtellingen van de opmerking zijn bijgewerkt (bevat gedetailleerde stemuitsplitsing)
- **Flagged** - De opmerking is door gebruikers gemarkeerd
- **UnFlagged** - De markeringen van de opmerking zijn verwijderd

### Moderatieacties
- **Pinned** - De opmerking is door een moderator vastgepind (bevat wie heeft vastgepind)
- **UnPinned** - De opmerking is door een moderator losgepind (bevat wie heeft losgepind)

### Meldingsgebeurtenissen
- **CreatedNotifications** - Er zijn meldingen aangemaakt voor de opmerking (bevat het aantal meldingen)
- **NotificationCreateFailure** - Het aanmaken van meldingen is mislukt
- **BadgeAwarded** - Er is een gebruikersbadge toegekend voor de opmerking (bevat de badge-naam)

### Gebeurtenissen voor vermeldings- en antwoordmeldingen

Deze gebeurtenissen noemen de persoon die de e-mail of melding zou ontvangen. Wanneer er niets is verzonden, staat in de kolom Details waarom.

- **MentionEmailSent** - Er is een e-mail verzonden naar een gebruiker die in de opmerking is genoemd
- **MentionEmailSkipped** - Er is geen e-mail verzonden naar een genoemde gebruiker (bevat de reden)
- **MentionHeldForApproval** - De vermeldings-e-mail wacht tot de opmerking is goedgekeurd
- **MentionNotificationCreated** - Een genoemde gebruiker heeft een in-app-melding ontvangen
- **MentionNotificationSkipped** - Een genoemde gebruiker heeft geen in-app-melding ontvangen (bevat de reden)
- **ReplyEmailSent** - De auteur van de opmerking waarop wordt gereageerd, is per e-mail geïnformeerd over dit antwoord
- **ReplyEmailSkipped** - De auteur van de opmerking waarop wordt gereageerd, is niet per e-mail geïnformeerd (bevat de reden)
- **ReplyNotificationSkipped** - De auteur van de opmerking waarop wordt gereageerd, heeft geen in-app-melding ontvangen (bevat de reden)

Redenen die worden getoond wanneer een e-mail of melding niet is verzonden:

- De gebruiker bestaat niet meer, of heeft geen e-mailadres
- De gebruiker heeft e-mailmeldingen uitgezet, of meldingen voor die thread uitgezet
- Een van de twee gebruikers heeft de andere geblokkeerd
- De gebruikers zitten niet in dezelfde SSO-groepen
- Het e-mailadres van de gebruiker staat op de suppressielijst na een bounce of spamklacht (zie [Email Suppression Management](/guide-notifications.html#email-suppression-management))
- Het e-mailadres van de gebruiker is example.com, dat geen e-mail kan ontvangen
- De opmerking is binnen 7 dagen gemarkeerd als spam, verwijderd of niet goedgekeurd
- De opmerking waarop wordt gereageerd, is anoniem geplaatst
- De gebruiker heeft op zijn eigen opmerking gereageerd
- De gebruiker is genoemd in het antwoord, dus hij heeft de vermeldings-e-mail ontvangen in plaats van een antwoord-e-mail
- De gebruiker had al een antwoordmelding voor de opmerking
- Het verzenden is 5 keer mislukt

Als de bezorging mislukt of een verzendlimiet wordt bereikt, wordt de e-mail in de wachtrij geplaatst voor opnieuw proberen en zegt het logboekitem dat ook.

### Publicatiegebeurtenissen
- **PublishedLive** - De opmerking is gepubliceerd naar live abonnees (bevat het aantal abonnees)

### Integratiegebeurtenissen
- **WebhookSynced** - De opmerking is gesynchroniseerd via webhook

### Spamregel-gebeurtenissen
- **SpamRuleMatch** - De opmerking kwam overeen met een aangepaste spamregel (bevat regeldetails)

### Lokalisatiegebeurtenissen
- **LocaleDetectedFromText** - De taal-locale is automatisch gedetecteerd uit de tekst van de opmerking (bevat de gedetecteerde taal en locale)

## Gebruiksscenario's voor Opmerkinglogboeken

Opmerkinglogboeken worden automatisch gegenereerd en bij elke opmerking opgeslagen. Ze bieden waardevolle inzichten voor:

- **Begrijpen van moderatiebeslissingen** - Zie precies waarom een opmerking is goedgekeurd, in beoordeling gehouden of als spam gemarkeerd
- **Debuggen van goedkeurings-/spamproblemen** - Volg de beslissingslogica wanneer opmerkingen zich niet gedragen zoals verwacht
- **Volgen van gebruikersgedragspatronen** - Monitor wijzigingen in trustfactor en verificatiestatus
- **Auditen van moderatoracties** - Bekijk welke acties moderators op specifieke opmerkingen hebben uitgevoerd
- **Onderzoeken van de effectiviteit van spamfilters** - Zie welke detectie-engines spam opvangen en welke niet
- **Problemen oplossen met integraties** - Verifieer webhook-syncs en meldingsbezorging

Deze logboeken helpen transparantie in het moderatieproces te waarborgen en helpen bij het fijnafstellen van het gedrag van je opmerkingensysteem.