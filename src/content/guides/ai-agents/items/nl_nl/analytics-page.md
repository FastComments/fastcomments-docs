Analytics is het cross-agent dashboard. Bereikbaar vanaf de AI Agents-pagina via het **Analytics**-tabblad (tenant-breed) of per agent via de **Analytics**-knop op de rij van elke agent.

### Filter

Een dropdown bovenaan - **Alle agents** of een specifieke agent. De rest van de pagina wordt dienovereenkomstig aangepast.

### Budgetgebruik

Vier voortgangsbalken die de uitgaven van de huidige periode tonen ten opzichte van het maximum:

- **Agent vandaag** (wanneer gefilterd op een specifieke agent) - dagelijkse agent-limiet.
- **Agent deze maand** - maandelijkse agent-limiet.
- **Account vandaag** - dagelijkse tenant-limiet.
- **Account deze maand** - maandelijkse tenant-limiet.

Wanneer een limiet niet is ingesteld, toont de balk "(geen limiet ingesteld)" en worden de ruwe uitgaven getoond.

### Dagelijkse kosten (laatste 30 dagen)

Een tabel met kosten per dag in de valuta van je tenant voor de geselecteerde scope. Handig voor het opsporen van:

- **Plotselinge kostenpieken** - meestal door een runaway loop of een virale reactie die triggers doet uitwaaieren.
- **Kostendrift** - geleidelijk stijgende dagelijkse kosten naarmate je community groeit.

### Uitgevoerde acties

Een uitsplitsing van actietypes over de huidige maand - "Heeft een reactie geschreven: 47", "Heeft een reactie als spam gemarkeerd: 12", enzovoort. Handig om te controleren dat de agent doet wat je verwachtte.

### Overgeslagen triggers (deze maand)

Aantallen gegroepeerd per [afwijzingsreden](#drop-reasons):

- Over agent dagelijks / agent maandelijks / account dagelijks / account maandelijks.
- Rate-limited.
- Concurrency verzadigd.

Als je hier afwijzingen ziet, stoot je agent tegen een budget- of ratelimiet aan en mist hij triggers waarop hij anders zou hebben gedraaid. Zie [Afwijzingsredenen](#drop-reasons).

### Dry-run vs live (deze maand)

- **Enabled runs** - aantal runs die deze maand echte acties hebben uitgevoerd.
- **Dry runs** - aantal runs in dry-run-modus deze maand.

Een nuttig afstemsignaal: een gloednieuwe agent die nog niet is gepromoveerd naar Enabled toont alleen dry runs. Een agent in Enabled met alléén nullen in deze sectie zit stil - of hij wordt niet getriggerd, of hij wordt buitengesloten, of zijn triggers zijn niet correct geconfigureerd.

### Top agents op maandkosten

Wanneer het filter **Alle agents** is, toont de pagina agents gerangschikt op kosten van maand-tot-datum. Je duurste agent opsporen is de eerste stap in kostenoptimalisatie - meestal is het antwoord "zijn [contextopties](#context-options) verstrakken" of "zijn [budgetlimiet](#budgets-overview) verlagen".

### Agents op of nabij hun limiet

Per-agent uitsplitsing van agents waarvan de uitgaven op of nabij hun per-agent limieten zitten in de huidige periode:

- **near cap** - boven een configureerbaar percentage van de limiet.
- **over cap** - daadwerkelijk afgekap, met `{count} dropped` triggers in die periode.

Klik in deze tabel op de agent om de limiet te verhogen, de scope te verkleinen of de agent te pauzeren.

### Accountoverzicht

Wanneer het filter **Alle agents** is:

- **Triggers vandaag** - aantal.
- **Triggers deze maand** - aantal.
- Voor elk: een `dropped`-suffix die toont hoeveel zijn overgeslagen.

### Valuta

Alle geldbedragen worden getoond in de valuta van je tenant.

### Wat deze pagina niet doet

- Het toont **geen per-actie kosten-uitsplitsingen** - die staan in [Run Detail View](#run-detail-view).
- Het toont **geen transcripts** of **LLM-responses**.
- Het laat je geen acties uitvoeren op agents - bewerken, pauzeren en verwijderen gebeuren allemaal vanuit de agentlijst / bewerkingspagina.