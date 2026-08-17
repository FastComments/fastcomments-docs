Agentomkostninger er **token-baseret**. Hvert LLM-kald returnerer et tokenantal, platformen konverterer dette til USD-cent ved hjælp af modellens per-token-rate, og centene faktureres mod agentens og lejerens budgetter.

### Hvad der faktureres

- **Alle LLM-kald**, inklusive kaldet der producerer nul værktøjsaktioner ("agenten besluttede at gøre ingenting"). Inferens betales selv når der ikke er nogen handling som resultat.
- **Dry-run-kald**. Dry-run er "ikke handle, men stadig kalde LLM" - LLM-kaldet koster det samme. Se [Dry-Run Mode](#dry-run-mode).
- **Replay-kald**. Replays er dry-run-kørsler mod historiske kommentarer. De koster tokens. Se [Test Runs (Replays)](#test-runs-replays).

### Hvad der ikke faktureres

- **Udløsere der aldrig producerer et LLM-kald.** Dropped-before-LLM tilfælde (over budget, rate begrænset, scope mismatch, fakturering ugyldig, loop forebyggelse) koster nul tokens. Se [Drop Reasons](#drop-reasons).
- **Værktøjsdisponering.** At kalde `pin_comment` eller et andet værktøj koster i sig selv ikke tokens - kun LLM-round-trippen gør.
- **`search_memory`.** Det er skrivebeskyttet og producerer ikke sin egen LLM-round-trip.

### Omkostning pr. kørsel

En enkelt agentkørsel kan kalde LLM flere gange - hvert værktøjskalds resultat føres tilbage til modellen, så den enten kan kalde et andet værktøj eller afslutte. Så `tokensUsed` på en kørsel er summen af alle LLM-round-trips i den kørsel.

De største bidragydere til tokenomkostninger pr. kørsel:

- **Lange [initial prompts](#personality-prompt) og [community guidelines](#community-guidelines)** - de indgår i hver kørsel.
- **[Context options](#context-options)** - trådkontekst, brugerhistorik, side metadata. Hver tilføjer tokens.
- **Selve kommentarteksten** - lange kommentarer koster mere.
- **Flere værktøjskald i én kørsel** - hver værktøjs resultatmeddelelse sendes tilbage til modellen.
- **Læse fra hukommelse** - `search_memory` returnerer op til 25 poster (begrænset til 8000 tegn i samlet indhold). De fleste af disse bytes går ind i den næste prompt.

**Max Tokens Per Trigger** (standard 20.000) begrænser **respons**-størrelsen pr. LLM-kald. Den begrænser ikke inputstørrelsen.

### Token-til-cent konvertering

Platformen anvender en enkelt per-lejer-pakke rate (`flexLLMCostCents` per `flexLLMUnit` tokens). Omkostning-per-token er på pakkens niveau, ikke per model - begge tilgængelige modeller ([GLM 5.1 and GPT-OSS Turbo](#choosing-a-model)) fakturerer til samme rate på en given pakke. [Run Detail View](#run-detail-view) viser omkostningen pr. kørsel i din valuta, når en kørsel er afsluttet.

### Hvor omkostninger registreres

Hver kørsel registrerer sit rå tokenantal og omkostning pr. kørsel. Daglige og månedlige totaler samles på [Analytics page](#analytics-page).

### Sådan læses omkostninger

- **Omkostning pr. kørsel**: [Run Detail View](#run-detail-view) -> `Cost`-feltet.
- **Daglig / månedlig samlet**: [Analytics page](#analytics-page) -> Budgetforbrug og daglige omkostningsdiagrammer.
- **Omkostning pr. handling**: også på Run Detail View, nyttig til justering når en agents værktøjs-loop er usædvanligt lang.

### Se også

- [Choosing a Model](#choosing-a-model) - den største indflydelse på omkostninger.
- [Context Options](#context-options) - hvor de ekstra omkostninger kommer fra.
- [Budgets Overview](#budgets-overview) - hårde grænser der forhindrer ukontrollerede omkostninger.

---