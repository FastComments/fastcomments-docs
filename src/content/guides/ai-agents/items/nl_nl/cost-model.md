Agentkosten zijn **token-gebaseerd**. Elke LLM-aanroep geeft een tokenaantal terug, het platform zet dat om naar USD-centen met behulp van het model's per-token tarief, en de centen worden in rekening gebracht op de budgetten van de agent en de tenant.

### Wat wordt gefactureerd

- **Alle LLM-aanroepen**, inclusief de aanroep die nul tool-acties oplevert ("de agent besloot niets te doen"). Inferentie wordt betaald zelfs wanneer er geen actie uit voortkomt.
- **Dry-run-aanroepen**. Dry-run betekent "niet handelen, maar toch het LLM aanroepen" - de LLM-aanroep kost hetzelfde. Zie [Dry-Run Mode](#dry-run-mode).
- **Replay-aanroepen**. Replays zijn dry-runs tegen historische opmerkingen. Ze kosten tokens. Zie [Test Runs (Replays)](#test-runs-replays).

### Wat niet wordt gefactureerd

- **Triggers die nooit een LLM-aanroep produceren.** Gevallen die worden afgebroken vóór de LLM (budget overschreden, gerate-limiteerd, scope mismatch, ongeldige facturatie, luspreventie) kosten nul tokens. Zie [Drop Reasons](#drop-reasons).
- **Tooldispatch.** Het aanroepen van `pin_comment` of een andere tool kost op zichzelf geen tokens - alleen de LLM round-trip doet dat.
- **`search_memory`.** Dit is read-only en produceert geen eigen LLM round-trip.

### Kosten per run

Een enkele agent-run kan meerdere keren de LLM aanroepen - ieder toolcall-resultaat wordt terug in het model gevoerd zodat het ofwel een andere tool kan aanroepen of kan afronden. Dus `tokensUsed` op een run is de som over alle LLM round-trips in die run.

De grootste bijdragers aan de tokenkosten per run:

- **Lange [initial prompts](#personality-prompt) en [community guidelines](#community-guidelines)** - ze worden bij elke run meegegeven.
- **[Context options](#context-options)** - threadcontext, gebruikersgeschiedenis, paginametadata. Elk voegt tokens toe.
- **De commentaartekst zelf** - lange opmerkingen kosten meer.
- **Meerdere tool-aanroepen in één run** - het resultaat van elke tool wordt terug naar het model gestuurd.
- **Memory-lezingen** - `search_memory` retourneert tot 25 records (gemaximeerd op 8000 tekens totale inhoud). Het merendeel van die bytes gaat in de volgende prompt.

**Max Tokens Per Trigger** (standaard 20.000) begrenst de **respons**grootte per LLM-aanroep. Het begrenst niet de inputgrootte.

### Token-naar-cent conversie

Het platform hanteert een enkel tarief per tenant-pakket (`flexLLMCostCents` per `flexLLMUnit` tokens). Kost-per-token is pakketniveau, niet per model - beide beschikbare modellen ([GLM 5.1 and GPT-OSS Turbo](#choosing-a-model)) worden tegen hetzelfde tarief op een gegeven pakket gefactureerd. De [Run Detail View](#run-detail-view) toont de kosten per run in uw valuta zodra een run is voltooid.

### Waar kosten worden geregistreerd

Elke run registreert zijn ruwe tokenaantal en kosten per run. Dagelijkse en maandelijkse totalen worden samengevat op de [Analytics page](#analytics-page).

### Hoe kosten te lezen

- **Kosten per run**: [Run Detail View](#run-detail-view) -> veld `Cost`.
- **Dagelijkse / maandelijkse aggregaat**: [Analytics page](#analytics-page) -> Budget usage en Daily cost grafieken.
- **Kosten per actie**: ook in de Run Detail View, nuttig om te finetunen wanneer de tool-lus van een agent ongewoon lang is.

### Zie ook

- [Choosing a Model](#choosing-a-model) - de grootste hefboom voor kosten.
- [Context Options](#context-options) - waar extra kosten vandaan komen.
- [Budgets Overview](#budgets-overview) - harde limieten die runaway-kosten voorkomen.