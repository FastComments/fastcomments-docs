FastComments handhaaft Artikel 17 van de EU Digital Services Act voor tenants in de EU-regio: **volledig geautomatiseerde gebruikersopschortingen zijn niet toegestaan**.

### Wat dat in de praktijk betekent

Wanneer uw tenant zich in de EU-regio bevindt, op het agent-bewerkingsformulier:

- Het selectievakje **Approvals** voor `ban_user` is **vergrendeld ingeschakeld** en kan niet worden uitgevinkt.
- Het label luidt: "EU DSA Artikel 17: gebruikersopschortingen vereisen menselijke beoordeling. 'Ban a user' is vergrendeld en kan niet volledig geautomatiseerd worden in de EU-regio."
- Een tooltip op de goedkeuringskolom vermeldt: "Vergrendeld door EU DSA Artikel 17 - volledig geautomatiseerde verbanningen zijn niet toegestaan in de EU-regio."

Wat u verder ook configureert, elke `ban_user`-aanroep van een agent op een tenant in de EU-regio gaat naar de [approvals inbox](#approval-workflow) voor menselijke beoordeling. De ban vindt niet plaats totdat een mens deze goedkeurt.

### Waarom dit op platformniveau wordt afgedwongen, niet op promptniveau

Systeemprompts kunnen worden genegeerd of omzeild door een model dat zich slecht gedraagt. Naleving van Artikel 17 is te belangrijk om te vertrouwen op het goede gedrag van het model; het moet een harde server-side poort zijn die de tool dispatcher zelf afdwingt. Dat is wat wij doen.

### Wat wel en niet door goedkeuring gaat

- **`ban_user`**: altijd onderhevig aan goedkeuring in de EU. Inclusief:
  - Zichtbare schorsingen (`shadowBan: false`).
  - Shadow bans (`shadowBan: true`).
  - Schorsingen met `deleteAllUsersComments: true`.
  - Schorsingen met `banIP: true`.
- Alle ban-varianten komen in de approvals inbox terecht met de motivatie en de vertrouwensscore van de agent; een mens keurt goed of wijst af.

De andere agent-tools (`mark_comment_spam`, `warn_user`, `lock_comment`, etc.) worden **niet** beïnvloed door Artikel 17. U kunt ze nog steeds automatiseren. Artikel 17 gaat specifiek over gebruikersopschortingen.

### Hoe zit het met niet-EU tenants

De vergrendeling geldt niet buiten de EU-regio. U kunt er toch voor kiezen om `ban_user` achter goedkeuring te plaatsen — we raden dat sterk aan voor de eerste weken van het leven van een moderatie-agent — maar het wordt niet afgedwongen.

### Shadow bans

Shadow bans worden beschouwd als schorsingen voor de doeleinden van Artikel 17 (de gebruiker kan posten maar hun inhoud is verborgen). Ze zijn op identieke wijze onderhevig aan goedkeuring als zichtbare schorsingen.

### Regiobepaling

De regio wordt bepaald op processniveau door de `REGION`-omgevingsvariabele op de FastComments-deployment (uitgelezen door `isEURegion()` in `models/constants.ts`). Er is geen per-tenant regioveld — de vergrendeling geldt voor elke tenant op een in de EU uitgerolde instantie. Als u uw data migreert van een niet-EU-deployment naar een EU-deployment, wordt de vergrendeling van kracht voor alle tenants op die instantie.

### Wat als alle beoordelaars niet beschikbaar zijn

De goedkeuring blijft in de inbox staan totdat er een beslissing wordt genomen. Deze verloopt automatisch 90 dagen na creatie. Er is geen pad "geen beoordelaar beschikbaar, doorgaan naar geautomatiseerde beslissing" — dat zou het doel van Artikel 17 ondermijnen.

Als uw community zo veel verkeer heeft dat EU-bans niet binnen een redelijke tijd kunnen worden beoordeeld, overweeg dan:

- Meer beoordelaars toe te voegen (zie [Approval Notifications](#approval-notifications)).
- De agent te laten overschakelen op het agressiever gebruiken van [`warn_user`](#tool-warn-user), aangezien waarschuwingen niet onder Artikel 17 vallen.
- De bereidheid van de agent om te bannen te verlagen door de [community guidelines](#community-guidelines) of de [initial prompt](#personality-prompt) aan te scherpen.

### Zie ook

- [Tool: ban_user](#tool-ban-user) voor wat `ban_user` doet en de destructieve opties achter extra opt-ins.
- [Approval Workflow](#approval-workflow) voor de volledige goedkeuringslevenscyclus.

---