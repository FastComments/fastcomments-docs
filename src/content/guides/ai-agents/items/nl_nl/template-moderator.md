**Sjabloon-ID:** `tos_enforcer`

Het Moderator-sjabloon is het aanbevolen startpunt als je doel is om de handmatige moderatielast te verminderen. Het beoordeelt nieuwe en gemarkeerde reacties en past jouw communityregels toe.

Je zult bijna altijd de **ingebouwde prompt willen uitbreiden** met concrete voorbeelden van wat jouw site wel en niet toestaat. Het eigen escalatiebeleid van het platform (waarschuwen vóór verbanning, geheugen raadplegen vóór verbanning) zit al ingebakken in de systeemprompt die de agent ontvangt, dus je hoeft het niet te herhalen.

### Triggers

- **Nieuwe reactie geplaatst** (`COMMENT_ADD`) - de agent bekijkt elke nieuwe reactie.
- **Reactie overschrijdt een markeerdrempel** (`COMMENT_FLAG_THRESHOLD`, standaarddrempel: 3) - de agent beoordeelt een reactie opnieuw die door andere gebruikers is gemarkeerd.

### Toegestane tools

- [`mark_comment_approved`](#tools-overview) - nuttig voor pre-moderatie-tenants waar de agent schone reacties vrijgeeft en de rest verbergt.
- [`mark_comment_spam`](#tools-overview)
- [`warn_user`](#tool-warn-user)
- [`ban_user`](#tool-ban-user)

Het kan geen reacties plaatsen, stemmen, vastpinnen, vergrendelen, badges uitreiken of e-mail versturen - de prompt is opzettelijk beperkt.

### Aanbevolen toevoegingen vóór livegang

- **Stel [Communityrichtlijnen](#community-guidelines) in.** Een paar zinnen geschreven beleid is voldoende; de agent past het bij elke run toe.
- **Zet `ban_user` achter een [goedkeuringsworkflow](#approval-workflow).** Dit staat standaard aan in de EU-regio (zie [EU DSA Artikel 17-naleving](#eu-dsa-compliance)) en wordt overal aanbevolen.
- **Overweeg ook `mark_comment_spam` achter goedkeuring te zetten** als je content hebt met een laag volume maar hoge inzet.
- **Zet `mark_comment_approved` achter goedkeuring als je pre-moderatie gebruikt.** Het goedkeuren van een slechte reactie plaatst deze voor lezers; zet het achter goedkeuring totdat de agent vertrouwen heeft verdiend via een dry-run.
- **Vink "Include commenter's trust factor, account age, ban history, and recent comments" aan** in [Contextopties](#context-options). Het model zal veel minder agressief waarschuwen wanneer het kan zien dat iemand een langdurige gebruiker met goede bedoelingen is.

### Aanbevolen dry-run-periode

Draai dit sjabloon in [dry-run](#dry-run-mode) gedurende ten minste een week tegen je echte verkeer voordat je het inschakelt als Enabled. Gebruik [Testruns (Replays)](#test-runs-replays) om ook een voorbeeld te bekijken tegen de afgelopen 30 dagen.