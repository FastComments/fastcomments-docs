Analytics is het agent-overstijgende dashboard. Te bereiken vanaf de AI Agents-pagina via het tabblad **Analytics** (tenant-breed) of per agent via de knop **Analytics** in de rij van elke agent.

### Filter

Een uitklapmenu bovenaan - **Alle agents** of een specifieke agent. De rest van de pagina wordt dienovereenkomstig aangepast.

### Budget usage

Vier voortgangsbalken die de uitgaven in de huidige periode tonen ten opzichte van de limiet:

- **Agent today** (wanneer gefilterd op een specifieke agent) - dagelijkse agentlimiet.
- **Agent this month** - maandelijkse agentlimiet.
- **Account today** - tenant-dagelijkse limiet.
- **Account this month** - tenant-maandelijkse limiet.

Wanneer een limiet niet is ingesteld, leest de balk "(no cap set)" en toont de ruwe uitgaven.

### Daily cost (last 30 days)

Een tabel met kosten per dag in de valuta van uw tenant voor het geselecteerde bereik. Nuttig om te ontdekken:

- **Sudden cost spikes** - meestal veroorzaakt door een uit de hand gelopen lus of een viraal commentaar dat triggers verspreidt.
- **Cost drift** - geleidelijke toename van de dagelijkse kosten naarmate je community groeit.

### Actions taken

Een uitsplitsing van actietypen over de huidige maand - "Reactie geplaatst: 47", "Reactie als spam gemarkeerd: 12", enzovoort. Nuttig om te controleren of de agent doet wat je verwacht.

### Triggers skipped (this month)

Tellingen gegroepeerd op [Redenen voor drops](#drop-reasons):

- Overschrijding van agent-daglimiet / agent-maandlimiet / account-daglimiet / account-maandlimiet.
- Door rate limiting beperkt.
- Gelijktijdigheid verzadigd.

Als je hier drops ziet, bereikt je agent een budget- of ratelimiet en mist hij triggers waarop hij anders zou zijn uitgevoerd. Zie [Redenen voor drops](#drop-reasons).

### Dry-run vs live (this month)

- **Enabled runs** - aantal runs die echte acties hebben uitgevoerd deze maand.
- **Dry runs** - aantal runs in dry-run-modus deze maand.

Een nuttig afstemsignaal: een gloednieuwe agent die nog niet naar 'Ingeschakeld' is gepromoveerd, zal alleen dry runs tonen. Een agent die 'Ingeschakeld' is maar in deze sectie overal nul aantallen heeft, zit inactief — ofwel wordt hij niet geactiveerd, ofwel wordt hij uitgefilterd, ofwel zijn de triggers niet correct geconfigureerd.

### Top agents by monthly cost

Wanneer het filter op **Alle agents** staat, toont de pagina agents gerangschikt op maand-tot-datum kosten. Het vinden van je duurste agent is de eerste stap in kostenoptimalisatie — meestal is het antwoord "verscherp de [contextopties](#context-options)" of "verlaag de [budgetlimiet](#budgets-overview)".

### Agents at or near their cap

Per-agent uitsplitsing van agents waarvan de uitgaven in de huidige periode bij of nabij hun per-agent limieten liggen:

- **near cap** - boven een configureerbaar percentage van de limiet.
- **over cap** - daadwerkelijk beperkt, met `{count} dropped` triggers in die periode.

Klik op de agent in deze tabel om de limiet te verhogen, het bereik te verkleinen, of deze te pauzeren.

### Account summary

Wanneer het filter op **Alle agents** staat:

- **Triggers today** - aantal.
- **Triggers this month** - aantal.
- Voor elk: een `dropped` suffix dat toont hoeveel zijn overgeslagen.

### Currency

Alle geldwaarden worden weergegeven in de valuta van uw tenant.

### What this page does not do

- Het toont geen **kostenuitsplitsing per actie** - die zijn te vinden op [Weergave met rundetails](#run-detail-view).
- Het toont geen **transcripten** of **LLM-responsen**.
- Het stelt je niet in staat om acties uit te voeren op agents - bewerken, pauzeren, verwijderen gebeuren allemaal vanuit de agentlijst / bewerkpagina.