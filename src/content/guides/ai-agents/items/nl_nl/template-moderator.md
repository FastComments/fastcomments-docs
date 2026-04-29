**Template ID:** `tos_enforcer`

Het Moderator-sjabloon is het aanbevolen uitgangspunt als je doel het verminderen van handmatige moderatiebelasting is. Het beoordeelt nieuwe en geflagde reacties en past jouw communityregels toe.

Je wilt bijna altijd de **ingebouwde prompt uitbreiden** met concrete voorbeelden van wat jouw site wel en niet toestaat. Het platform-eigen escalatiebeleid (waarschuwen vóór verbanning, geheugen doorzoeken vóór verbanning) is al ingebakken in de system prompt die de agent ontvangt, dus dat hoef je niet te herhalen.

### Triggers

- **New comment posted** (`COMMENT_ADD`) - de agent bekijkt elke nieuwe reactie.
- **Comment crosses a flag threshold** (`COMMENT_FLAG_THRESHOLD`, default threshold: 3) - de agent evalueert opnieuw een reactie die door andere gebruikers is geflagd.

### Allowed tools

- [`mark_comment_approved`](#tools-overview) - nuttig voor pre-moderatie tenants waarbij de agent schone reacties vrijgeeft en de rest verbergt.
- [`mark_comment_spam`](#tools-overview)
- [`warn_user`](#tool-warn-user)
- [`ban_user`](#tool-ban-user)

Hij kan geen reacties plaatsen, stemmen, vastpinnen, vergrendelen, badges toekennen of e-mail verzenden - de prompt is opzettelijk beperkt.

### Recommended additions before going live

- **Set [Gemeenschapsrichtlijnen](#community-guidelines).** Een paar zinnen met geschreven beleid is voldoende; de agent past dit toe bij elke run.
- **Gate `ban_user` behind [goedkeuring](#approval-workflow).** Dit staat standaard aan in de EU-regio (zie [Naleving EU DSA Artikel 17](#eu-dsa-compliance)) en wordt overal aangeraden.
- **Overweeg ook om `mark_comment_spam` achter goedkeuring te plaatsen** als je weinig volume maar hoge risico-inhoud hebt.
- **Plaats `mark_comment_approved` achter goedkeuring als je pre-moderatie uitvoert.** Het goedkeuren van een slechte reactie plaatst deze voor lezers; zet er een hek om totdat de agent vertrouwen heeft verdiend via dry-run.
- **Vink "Include commenter's trust factor, account age, ban history, and recent comments" aan** in [Contextopties](#context-options). Het model zal veel minder agressief waarschuwen wanneer het kan zien dat iemand een langdurige goedbedoelde gebruiker is.

### Recommended dry-run window

Draai dit sjabloon in [testmodus](#dry-run-mode) gedurende minstens een week tegen je echte verkeer voordat je het op Enabled zet. Gebruik [Testuitvoeringen (Replays)](#test-runs-replays) om ook te previewen tegen de afgelopen 30 dagen.