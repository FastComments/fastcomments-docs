Agentomkostninger er **token-baserede**. Hvert LLM-opkald returnerer et tokenantal, platformen konverterer det til USD cents ved hjælp af modellens pris per token, og centene belastes agentens og tenantens budgetter.

### Hvad der faktureres

- **Alle LLM-opkald**, inklusive opkaldet der producerer nul tool-handlinger ("agenten besluttede ikke at gøre noget"). Inferens betales selv når ingen handling resulterer.
- **Dry-run-kald**. Dry-run betyder "gør ikke noget, men kald stadig LLM" - LLM-opkaldet koster det samme. Se [Dry-Run-tilstand](#dry-run-mode).
- **Replay-kald**. Replays er dry-run-kørsler mod historiske kommentarer. De koster tokens. Se [Testkørsler (Replays)](#test-runs-replays).

### Hvad der ikke faktureres

- **Triggers der aldrig producerer et LLM-opkald.** Droppet-før-LLM-tilfælde (overskredet budget, rate-begrænset, scope-mismatch, ugyldig fakturering, loop-forebyggelse) koster nul tokens. Se [Drop-årsager](#drop-reasons).
- **Tool-dispatch.** At kalde `pin_comment` eller ethvert andet værktøj koster i sig selv ikke tokens - kun LLM-roundtrippen gør.
- **`search_memory`.** Det er skrivebeskyttet og producerer ikke sin egen LLM-roundtrip.

### Omkostning per kørsel

En enkelt agentkørsel kan kalde LLM flere gange - resultatet af hvert tool-kald føres tilbage til modellen, så den enten kan kalde et andet værktøj eller afslutte. Så `tokensUsed` på en kørsel er summen over alle LLM-roundtrips i den kørsel.

De største bidragydere til token-omkostningen pr. kørsel:

- **Lange [indledende prompts](#personality-prompt) og [fællesskabsretningslinjer](#community-guidelines)** - de indgår i hver kørsel.
- **[Kontekstmuligheder](#context-options)** - tråd-kontekst, brugerhistorik, sidemetadata. Hver tilføjer tokens.
- **Selve kommentarteksten** - lange kommentarer koster mere.
- **Flere tool-kald i én kørsel** - resultatbeskeden fra hvert værktøj sendes tilbage til modellen.
- **Memory-læsninger** - `search_memory` returnerer op til 25 poster (begrænset til 8000 tegn i alt). De fleste af disse bytes går ind i det næste prompt.

**Max Tokens Per Trigger** (default 20,000) caps the **response** size per LLM call. It does not cap the input size.

### Token-til-cent konvertering

Platformen anvender en enkelt per-tenant-package sats (`flexLLMCostCents` per `flexLLMUnit` tokens). Pris per token er på pakkeniveau, ikke per model - begge tilgængelige modeller ([GLM 5.1 and GPT-OSS Turbo](#choosing-a-model)) fakturerer til samme sats på en given pakke. [Kørselsdetaljer](#run-detail-view) viser omkostningen pr. kørsel i din valuta, når en kørsel er færdig.

### Hvor omkostningen registreres

Hver kørsel registrerer sit rå tokenantal og omkostning pr. kørsel. Daglige og månedlige totaler rulles op på [Analytics-siden](#analytics-page).

### Sådan læser du omkostninger

- **Omkostning pr. kørsel**: [Kørselsdetaljer](#run-detail-view) -> `Cost`-feltet.
- **Dagligt / månedligt samlet**: [Analytics-siden](#analytics-page) -> Budgetforbrug og diagrammer for daglige omkostninger.
- **Omkostning pr. handling**: også i [Kørselsdetaljer](#run-detail-view), nyttigt til tuning når en agentens tool-loop er usædvanligt lang.

### Se også

- [Valg af model](#choosing-a-model) - den største faktor for omkostninger.
- [Kontekstmuligheder](#context-options) - hvor den tilføjede omkostning kommer fra.
- [Budgetoversigt](#budgets-overview) - hårde grænser der forhindrer løbende omkostninger.