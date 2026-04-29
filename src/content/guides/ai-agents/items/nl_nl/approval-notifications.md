Wanneer de agent een goedkeuring in de wachtrij zet, stelt het platform beoordelaars via e-mail op de hoogte. Twee instellingen op het bewerkingsformulier bepalen dit: **wie** wordt geïnformeerd en **hoe vaak**.

### Wie: meldingsmodus

Twee modi:

- **All admins and moderators** (default) - elke account-eigenaar, super admin en comment moderator admin op de tenant is een kandidaat-beoordelaar.
- **Specific users** - kies handmatig een lijst uit een dual-list picker op het bewerkingsformulier.

Hoe dan ook moet een kandidaat-beoordelaar een account op de tenant hebben en een geldig e-mailadres om meldingen te ontvangen.

### Hoe vaak: frequentie per gebruiker

Elke kandidaat-beoordelaar stelt in **zijn of haar eigen profiel** de persoonlijke notificatiefrequentie voor agent-goedkeuringen in:

- **Immediate** (default) - één e-mail per lopende goedkeuring, verzonden zodra de goedkeuring is aangemaakt.
- **Hourly** - één samenvattende e-mail per uur met alle goedkeuringen die in dat uur in de wachtrij zijn gezet.
- **Daily** - één samenvattende e-mail per 24 uur.
- **Disabled** - geen e-mails. De gebruiker kan nog steeds goedkeuringen beoordelen via de inbox-UI; ze ontvangen alleen geen melding.

De gebruiker verandert deze instelling in zijn of haar eigen profiel, niet op het agent-bewerkingsformulier. Dit is bewust zo ontworpen: één tenant kan tien agents hebben, en een moderator zou niet zijn of haar voorkeursfrequentie afzonderlijk voor elke agent moeten moeten instellen.

### Cronjobs die samenvattingen aansturen

- **`hourly-agent-approval-digest`** - draait elk uur, groepeert goedkeuringen die sinds ieders laatste samenvatting zijn toegevoegd, en stuurt één e-mail per gebruiker.
- **`daily-agent-approval-digest`** - hetzelfde, dagelijks.
- **`agent-approval-reaper`** - ruimt goedkeuringen op die ouder zijn dan 90 dagen, ongeacht de status.

De hourly- en daily-samenvattingscrons zijn per ontvanger afgebakend: een gebruiker met hourly-frequentie wordt door de hourly-cron verwerkt en door de daily-cron overgeslagen (en omgekeerd). Gebruikers met immediate-frequentie worden door het approval-create pad geïnformeerd, niet door de crons.

### Dedup-status

Het platform houdt bij welke gebruikers al per e-mail geïnformeerd zijn over elke goedkeuring. Zodra een gebruiker is op de hoogte gesteld (onmiddellijk of in een samenvatting), ontvangt die gebruiker niet opnieuw een e-mail voor dezelfde goedkeuring - zelfs niet als hij of zij halverwege de cyclus van immediate naar daily verandert.

### Goedkeuren vanuit de e-mail

Elke notificatiemail bevat een één-klik ondertekende aanmeldingslink die de beoordelaar rechtstreeks naar de detailpagina van de goedkeuring brengt, al geauthenticeerd. Vanaf daar kunnen ze goedkeuren, afwijzen of de [Prompts verfijnen](#refining-prompts)-flow openen.

### Wat als er geen admins zijn

Als `notifyMode` `All admins and moderators` is maar de tenant geen super admins, comment moderator admins of account-eigenaren met geldige e-mails heeft, logt het platform een waarschuwing en wordt de goedkeuring toch in de wachtrij geplaatst — alleen krijgt niemand er een melding van. De goedkeuring blijft in de inbox staan totdat iemand er toevallig naar kijkt.

Als `notifyMode` `Specific users` is maar je hebt geen gebruikers geselecteerd, is het resultaat hetzelfde.

### Wat als factureringsmeldingen zijn uitgeschakeld

[Budget Alerts](#budget-alerts) - de budget-gerelateerde e-mails - gaan naar de billing admins **ongeacht de per-gebruiker notificatievoorkeur**. Dit is bedoeld: budgetoverschrijdingen hebben invloed op kosten, en de billing-eigenaar moet dit weten.

Goedkeuringsmeldingen houden alleen rekening met de per-gebruiker agent-approval frequentie-instelling. Ze controleren niet de bredere opt-out voor admin-meldingen - een gebruiker die zich heeft afgemeld voor admin-meldingen ontvangt nog steeds goedkeuringsmails als hij of zij op de beoordelaarslijst staat, tenzij zijn of haar agent-approval frequentie is ingesteld op **Disabled**.

### Zie ook

- [Approval Workflow](#approval-workflow) voor de volledige levenscyclus van een goedkeuring.
- [Prompts verfijnen](#refining-prompts) voor de workflow "Ik blijf hetzelfde soort fout goedkeuren".