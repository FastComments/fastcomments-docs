**Dry Run** is de veilige modus waarin elke nieuwe agent start. De agent draait end-to-end, behalve in het gedeelte waarin hij wijzigingen in je community aanbrengt.

### Wat er in Dry Run wordt uitgevoerd

- Triggers worden normaal geactiveerd.
- De prompt van de agent, de [communityrichtlijnen](#community-guidelines) en de [context](#context-options) worden samengesteld.
- Het LLM wordt aangeroepen.
- Het model selecteert tool-aanroepen en levert rechtvaardigingen en vertrouwensscores.
- De run wordt geregistreerd met een **Dry Run**-badge zodat deze duidelijk te onderscheiden is van live runs.

### Wat er niet wordt uitgevoerd in Dry Run

- Er wordt geen reactie geplaatst, geen stem uitgebracht, geen reactie vastgezet/losgemaakt/vergrendeld/ontgrendeld.
- Geen reactie wordt als spam gemarkeerd, goedgekeurd of beoordeeld.
- Geen gebruiker wordt verbannen, gewaarschuwd of een badge toegekend.
- Er wordt geen e-mail verzonden.
- Er wordt geen geheugen geschreven. (Ja — ook geheugen. Dry-run agents kunnen de gedeelde geheugenpool niet vervuilen met hypothetische beslissingen.)
- Er worden geen webhooks geactiveerd voor tool-acties. (De trigger-level `trigger.succeeded` / `trigger.failed` webhooks worden nog steeds geactiveerd en de payload bevat `wasDryRun: true`. Zie [Webhook Payloads](#webhook-payloads).)

### Wat het kost

Dry Run-uitvoeringen doen **dezelfde LLM-aanroep** als een Enabled-run zou doen. Tokens worden in rekening gebracht, [budgetlimieten](#budgets-overview) zijn van toepassing, en de runs tellen mee voor de dagelijkse/maandelijkse limieten per agent en per tenant.

Die kosten zijn de prijs voor een getrouwe preview. Een modus die de LLM-aanroep overslaat zou je geen enkele aanwijzing geven over hoe de agent zou handelen.

### Dry-runresultaten lezen

In de [Run History](#run-history) worden dry-run runs gemarkeerd met de **Dry Run**-badge in de statuskolom. De acties binnen elke run lijken identiek aan live-acties - dezelfde toolnaam, dezelfde argumenten, dezelfde rechtvaardiging en dezelfde vertrouwensscore - behalve dat geen van deze acties daadwerkelijk heeft plaatsgevonden.

De [Analytics-pagina](#analytics-page) verdeelt "dry-run vs live" runs per maand zodat je kunt zien hoeveel van je token-uitgaven naar observatie ging.

### Overschakelen van Dry Run

Bewerk de agent en wijzig **Status** naar **Enabled**. De volgende trigger draait live.

Je kunt ook de andere kant op schakelen - van Enabled terug naar Dry Run - als de agent dingen begint te doen die je niet bevalt. Er zijn geen consequenties.

### Replays dwingen Dry Run

De functie [Test Runs (Replays)](#test-runs-replays) draait de agent altijd in dry-run tegen historische reacties, ongeacht de opgeslagen status van de agent. Replays kunnen geen echte acties ondernemen op eerdere reacties. Dit is opzettelijk zo ontworpen - replay is een preview-tool, geen moderatietool.