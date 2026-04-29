**Sjabloon-ID:** `tos_enforcer`

De Moderator-sjabloon is het aanbevolen startpunt als uw doel is het verminderen van handmatige moderatiebelasting. Het beoordeelt nieuwe en gemarkeerde reacties en past uw communityregels toe.

### Ingebouwde initiële prompt

[inline-code-attrs-start title = 'Moderator-sjabloon initiële prompt'; type='text' inline-code-attrs-end]
[inline-code-start]
You are a terms-of-service enforcement agent. Review each new comment against the community guidelines. Mark clear spam or policy violations. Issue warnings for first-offense borderline content. Escalate ban decisions only for repeat or severe violations. If a comment is clearly within the rules, approve it so it becomes visible (relevant for pre-moderation tenants). Stay out of political or subjective debates, focus on the rules as written.
[inline-code-end]

U zult vrijwel altijd deze prompt willen **aanvullen** met concrete voorbeelden van wat uw site wel en niet toestaat. Het platform's eigen escalatiebeleid (waarschuwen vóór verbanning, geheugen doorzoeken vóór het verbannen) is al ingebakken in de systeemprompt die de agent ontvangt, dus u hoeft het niet te herhalen.

### Triggers

- **Nieuwe reactie geplaatst** (`COMMENT_ADD`) - de agent kijkt naar elke nieuwe reactie.
- **Reactie overschrijdt een flag-drempel** (`COMMENT_FLAG_THRESHOLD`, standaard drempel: 3) - de agent evalueert een reactie opnieuw die door andere gebruikers is gemarkeerd.

### Toegestane tools

- [`mark_comment_approved`](#tools-overview) - nuttig voor pre-moderation tenants waar de agent schone reacties vrijgeeft en de rest verbergt.
- [`mark_comment_spam`](#tools-overview)
- [`warn_user`](#tool-warn-user)
- [`ban_user`](#tool-ban-user)

Het kan geen reacties plaatsen, stemmen, vastpinnen, vergrendelen, badges toekennen of e-mail verzenden - de prompt is opzettelijk beperkt.

### Aanbevolen aanvullingen voordat u live gaat

- **Stel [Community-richtlijnen](#community-guidelines) in.** Enkele zinnen geschreven beleid zijn genoeg; de agent past dit bij elke uitvoering toe.
- **Zet `ban_user` achter [goedkeuring](#approval-workflow).** Dit staat standaard aan in de EU-regio (zie [EU DSA Artikel 17-conformiteit](#eu-dsa-compliance)) en wordt overal aanbevolen.
- **Overweeg ook om `mark_comment_spam` achter goedkeuring te zetten** als u weinig volume maar hoge risico's hebt met betrekking tot content.
- **Zet `mark_comment_approved` achter goedkeuring als u pre-moderation runt.** Het goedkeuren van een slechte reactie plaatst deze voor lezers; zet het achter een goedkeuringsstap totdat de agent vertrouwen heeft verdiend via dry-run.
- **Vink "Include commenter's trust factor, account age, ban history, and recent comments" aan** in [Contextopties](#context-options). Het model zal veel minder agressief waarschuwen wanneer het kan zien dat iemand een langlopende goedbedoelde gebruiker is.

### Aanbevolen dry-run-venster

Draai deze sjabloon in [dry-run](#dry-run-mode) gedurende minstens een week tegen uw echte verkeer voordat u overschakelt naar Enabled. Gebruik [Test Runs (Replays)](#test-runs-replays) om ook een voorbeeld te zien tegen de afgelopen 30 dagen.