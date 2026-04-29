Een **test run** (ook wel **replay** genoemd) voert de agent uit tegen een venster met historische reacties **zonder echte acties uit te voeren**. Het is de snelste manier om het gedrag van een agent vooraf te bekijken voordat u live gaat.

Bereikbaar vanaf de paginalijst met agents via de knop **Test run** in elke regel van een agent.

### Wat het doet

Het platform:

1. Selecteert een steekproef van historische reacties die overeenkomen met de scope van de agent, in het door u gekozen venster.
2. Voert voor elke reactie de agent end-to-end uit alsof de reactie zojuist geplaatst is - dezelfde context, dezelfde LLM-aanroep, dezelfde toolselectie, dezelfde rechtvaardigingen en betrouwbaarheidscores.
3. Registreert elke run als een dry-run, getagd zodat deze gegroepeerd blijft met de replay waaruit hij afkomstig is en uitgesloten wordt van live-run weergaven.
4. **Vergelijkt** het oordeel van de agent met wat er daadwerkelijk met de reactie gebeurd is - is deze later goedgekeurd, als spam gemarkeerd, verwijderd, geblokkeerd door een spam-engine, enz.

Het resultaat is een per-reactie diff: "De replay-agent zou dit als spam markeren, maar de reactie is momenteel goedgekeurd en schoon."

### Configuratie

De test-run pagina heeft één invoerveld:

- **Days of historical comments to evaluate** - een numeriek `days` veld tussen 1 en 90. Oudere reacties komen niet in aanmerking.

De steekproefgrootte en harde limiet worden **niet in de UI weergegeven** - beide zijn server-side standaardinstellingen die per plan worden toegepast. De pagina toont informatieve velden:

- **Matching comments in window** - hoeveel reacties in overweging zouden worden genomen.
- **Up to N comments from this window will be processed** - de effectieve steekproefgrootte gegeven de server-side limiet.
- **Estimated cost** - in de valuta van uw tenant.

### Snelheidslimiet

Elke gebruiker is beperkt tot **10 test runs per 24 uur** (rate-limited via key `replay-create:${requestedBy}`). De knop toont een tooltip wanneer u de limiet hebt bereikt ("You\'ve reached 10 test runs in the last 24 hours.").

### Gelijktijdigheid

Er kan slechts één replay tegelijk actief zijn per agent. Het starten van een tweede replay terwijl er al een actief is, leidt u om naar de replay die in uitvoering is.

### Resultaten lezen

Wanneer de replay voltooid is, toont de resultaatpagina tabbladen:

- **Deltas** (standaard-actief) - de replay-agent geeft een ander oordeel dan de werkelijkheid. (Meest interessant - "de agent zou deze reactie als spam hebben gemarkeerd, maar de reactie was goedgekeurd en in orde".)
- **Matches** - het oordeel van de replay-agent komt overeen met wat er daadwerkelijk gebeurd is. (Bemoedigend - de agent is het eens met de werkelijkheid.)
- **No action** - de replay-agent besloot niets te doen. (Soms het juiste antwoord; soms heeft de agent iets gemist.)
- **All** - elk resultaat ongeacht classificatie.

Voor elke reactie in elk tabblad:

- **Prior outcome** - de classificatie van wat er daadwerkelijk gebeurd is: **POSITIVE**, **NEGATIVE**, of **INDETERMINATE**, met **Bewijs** ("Comment marked deleted at {date}", "Engine: bayes", enzovoort).
- **Replay agent would** - de actie die de agent koos.
- **Why** - de rechtvaardiging.
- **Confidence** - weergegeven als een percentage.

### Waarom replays dry-run afdwingen

Een replay tegen een reactie die vier maanden geleden verwijderd is, zou deze niet achteraf moeten verwijderen - deze is al verwijderd. Een replay tegen een reactie die de agent nu zou willen goedkeuren, zou de huidige status van de reactie niet moeten veranderen. Replay is een preview-tool. Het afdwingen van dry-run is wat het veilig maakt om een replay uit te voeren tegen elk willekeurig historisch venster.

### Reproduceerbaarheid

Replays bevriezen de configuratie van de agent op het moment dat de replay gestart is. Latere bewerkingen aan de agent veranderen de resultaten van de replay niet - de resultaatpagina blijft stabiel als een record van wat *die* versie van de agent zou hebben gedaan.

### Wanneer budgetten een replay stoppen

Replays zijn onderhevig aan:

- Hun eigen **harde limiet** (ingesteld op het replay-formulier).
- De dagelijkse en maandelijkse **budgetlimieten** van de agent.
- De dagelijkse en maandelijkse **budgetlimieten** van de tenant.

De eerste limiet die wordt bereikt, breekt de replay af met een specifieke foutcode. Alle per-reactie resultaten die vóór de afbreking zijn geproduceerd, worden bewaard in [Run History](#run-history).

### Hoe replays werken

Replays draaien op de achtergrond, niet synchroon. Nadat u op "Start test run" hebt geklikt, wordt de replay in de wachtrij geplaatst en pakt een worker deze op. Een lange replay kan enkele minuten duren. De resultaatpagina polt en toont voortgang (aantal verwerkt, gemaakte kosten tot nu toe) terwijl het loopt.

Als een worker halverwege een replay crasht, zet het platform de replay automatisch opnieuw in de wachtrij zodat het bij de volgende keer wordt voortgezet. Een korte onderbreking laat een replay nooit achter als verweesd.

### Wat replay niet doet

- **Negeert [trigger delays](#trigger-deferred-delay).** Replays draaien onmiddellijk, niet 30 minuten later.
- **Schrijft niet naar geheugen.** Replay-agents slaan geen geheugennotities op, zelfs als hun logica dat normaal wel zou doen.
- **Feuert geen webhooks af.** Door replay geproduceerde triggers genereren geen `trigger.succeeded` / `trigger.failed` webhook events.
- **Sluit niet reeds gereplayde reacties uit.** Het uitvoeren van een tweede replay tegen hetzelfde venster behandelt dezelfde reacties.

### Zie ook

- [Refining Prompts](#refining-prompts) - de iteratieve bewerkingsworkflow die goed samengaat met replays.
- [Dry-Run Mode](#dry-run-mode) - hetzelfde idee, tegen liveverkeer.