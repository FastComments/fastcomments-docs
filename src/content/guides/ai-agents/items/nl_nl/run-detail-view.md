Klikken op **Weergeven** in een rij in [Uitvoeringsgeschiedenis](#run-history) opent de detailpagina per uitvoering. Dit is waar u de redenering van de agent leest en diens beslissingen beoordeelt.

### Bovenaan: samenvatting van de uitvoering

- **Agent** - welke agent draaide.
- **Wanneer** - tijdstempel.
- **Status** - Gestart / Geslaagd / Fout, plus de **Proefrun** badge indien van toepassing.
- **Kosten** - kosten per uitvoering in de valuta van uw tenant.
- **Kosten per actie** - kosten gedeeld door het aantal niet in afwachting zijnde acties, nuttig om uitzonderlijk dure uitvoeringen op te sporen.

### Uitgevoerde acties

Een lijst, in volgorde, van elke tool-aanroep die de uitvoering deed. Elke vermelding toont:

- **Actielabel** - "Heeft een reactie geschreven", "Markeerde een reactie als spam", "Verbannen gebruiker", enz. Het label wordt gemapt vanaf de actie-type enum.
- **Referentie-ID** - de getroffen reactie-, gebruiker- of badge-ID, weergegeven als monospace-tekst (geen hyperlink).
- **Redenering van de agent** - de rechtvaardiging die de agent bij de oproep gaf.
- **Vertrouwen** - de door de agent zelf ingeschatte zekerheid, weergegeven als een percentage.
- **Badge in afwachting van goedkeuring** - als de actie in de [goedkeuringsinbox](#approval-workflow) in de wachtrij staat in plaats van uitgevoerd te worden.

Als de uitvoering nul acties uitvoerde, staat er in deze sectie: "Geen acties werden tijdens deze uitvoering ondernomen."

### LLM-transcript

Onder de acties, het volledige transcript van het gesprek van de agent met de LLM:

- **Systeem** - de system prompt (platform suffix + uw initiële prompt + communityrichtlijnen).
- **Gebruiker** - het contextbericht dat de trigger beschrijft.
- **Assistent** - de reacties van het model, inclusief tool-aanroepen.
- **Tool** - het toolresultaat dat terug naar het model werd gevoerd (bijv. wat `search_memory` teruggaf).

Lange berichten zijn inklapbaar; klik **Uitvouwen** / **Samenvouwen** om te bekijken.

### Transcripten lezen

Het transcript is de belangrijkste pagina voor het afstemmen. Wanneer de agent een beslissing neemt waar u het niet mee eens bent, lees het dan terug om te zien:

- Wat het model **zag** (het contextbericht van de Gebruiker).
- Wat het model **besloot** (de Assistent-tooloproepen).
- Wat het model **overwoog** (eventuele toolresultaten - bijv. heeft de agent daadwerkelijk `search_memory` aangeroepen en vond het iets voordat er een verbanning plaatsvond).

Als het model consequent hetzelfde soort fout maakt, bewerk de [initiële prompt](#personality-prompt) - of gebruik [Prompt verfijnen](#refining-prompts) vanuit een afgewezen goedkeuring.

### Actiereferenties

De referentie-ID's worden weergegeven als monospace-tekst (geen hyperlinks):

- Reacties: de reactie-ID.
- Gebruikers: de gebruiker-ID.
- Badges: de badge-ID.

U kunt de ID kopiëren om het betreffende record op de relevante moderatie-/adminpagina op te zoeken.

### Wat ontbreekt bij een proefrun

Proefrun-uitvoeringen tonen dezelfde acties, rechtvaardigingen en vertrouwensscores. Het enige verschil is de **Proefrun** badge op de statusregel. De referentie-ID's voor reacties / gebruikers / badges worden nog steeds weergegeven - de agent heeft ze alleen niet beïnvloed.

### Fouten

Voor uitvoeringen in de `Error` state toont de detailpagina het onderliggende foutbericht. Veelvoorkomende fouten:

- **Geen LLM API-sleutel geconfigureerd** - tenant- of platformmisconfiguratie.
- **Time-out bij LLM-aanroep** - de LLM-provider was traag of niet beschikbaar.
- **Fout bij tool-dispatch** - de agent koos een tool met onjuiste argumenten (bijv. een reactie-ID die niet meer bestaat).
- **Budget halverwege uitgeput** - de limiet van de agent werd bereikt terwijl de uitvoering bezig was. De uitvoering werd stopgezet.

Fouten rollen gedeeltelijke acties niet terug - alle tooloproepen die vóór de fout zijn voltooid, blijven bestaan.