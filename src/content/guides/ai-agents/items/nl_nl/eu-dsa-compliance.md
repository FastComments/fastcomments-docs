FastComments handhaaft Artikel 17 van de EU Digital Services Act voor tenants in de EU-regio: **volledig geautomatiseerde gebruikersschorsingen zijn niet toegestaan**.

### Wat dat in de praktijk betekent

Wanneer je tenant in de EU-regio zit, geldt op het agent-bewerkingsformulier:

- Het **Approvals**-vinkje voor `ban_user` is **vergrendeld aan** en kan niet worden uitgevinkt.
- Het label luidt: "EU DSA Artikel 17: gebruikersschorsingen vereisen menselijke beoordeling. 'Ban a user' is vergrendeld aan en kan niet volledig worden geautomatiseerd in de EU-regio."
- Een tooltip op de goedkeuringskolom luidt: "Vergrendeld aan door EU DSA Artikel 17 - volledig geautomatiseerde bans zijn niet toegestaan in de EU-regio."

Wat je ook configureert, elke `ban_user`-aanroep van een agent op een tenant in de EU-regio gaat naar de [approvals inbox](#approval-workflow) voor menselijke beoordeling. De ban wordt pas uitgevoerd nadat een mens deze heeft goedgekeurd.

### Waarom dit op platformniveau wordt afgedwongen, en niet op promptniveau

Systeemprompts kunnen worden genegeerd of omzeild door een model dat zich slecht genoeg gedraagt. Naleving van Artikel 17 is te belangrijk om af te hangen van het goede gedrag van het model; het moet een harde server-side poort zijn die de tool dispatcher zelf afdwingt. En dat is precies wat we doen.

### Wat wel en niet door goedkeuring gaat

- **`ban_user`**: altijd afgeschermd in de EU. Inclusief:
  - Zichtbare bans (`shadowBan: false`).
  - Shadow bans (`shadowBan: true`).
  - Bans met `deleteAllUsersComments: true`.
  - Bans met `banIP: true`.
- Alle ban-varianten belanden in de approvals inbox met de redenering en het vertrouwen van de agent; een mens keurt goed of af.

De andere agent-tools (`mark_comment_spam`, `warn_user`, `lock_comment`, enz.) worden **niet** beïnvloed door Artikel 17. Je kunt ze nog steeds automatiseren. Artikel 17 gaat specifiek over gebruikersschorsingen.

### Wat met niet-EU-tenants

De vergrendeling geldt niet buiten de EU-regio. Je kunt ervoor kiezen om `ban_user` toch achter goedkeuring te plaatsen - we raden dat sterk aan voor de eerste weken van het leven van elke moderatie-agent - maar het wordt niet afgedwongen.

### Shadow bans

Shadow bans gelden als schorsingen in het kader van Artikel 17 (de gebruiker kan posten maar de content is verborgen). Ze worden op dezelfde manier afgeschermd als zichtbare bans.

### Regiodetectie

De regio wordt op procesniveau bepaald door de omgevingsvariabele `REGION` van de FastComments-implementatie (gelezen door `isEURegion()` in `models/constants.ts`). Er is geen regioveld per tenant - de vergrendeling geldt voor elke tenant op een in de EU geïmplementeerde instantie. Als je je data migreert van een niet-EU-implementatie naar een EU-implementatie, treedt de vergrendeling in werking voor alle tenants op die instantie.

### Wat als alle reviewers niet beschikbaar zijn

De goedkeuring blijft in de inbox liggen totdat er een beslissing is genomen. Deze verloopt automatisch 90 dagen na aanmaak. Er is geen pad "geen reviewer beschikbaar, val terug op geautomatiseerde beslissing" - dat zou het doel van Artikel 17 tenietdoen.

Als je community zo hoogvolume is dat EU-bans niet binnen een redelijke tijd kunnen worden beoordeeld, overweeg dan:

- Meer reviewers toevoegen (zie [Approval Notifications](#approval-notifications)).
- De agent agressiever [`warn_user`](#tool-warn-user) laten gebruiken, aangezien waarschuwingen niet onder Artikel 17 vallen.
- De ban-honger van de agent verlagen door de [community guidelines](#community-guidelines) of de [initial prompt](#personality-prompt) aan te scherpen.

### Zie ook

- [Tool: ban_user](#tool-ban-user) voor wat `ban_user` doet en de destructieve opties achter extra opt-ins.
- [Approval Workflow](#approval-workflow) voor de volledige goedkeuringslevenscyclus.