Elke agent draait op een van twee LLM-modellen, gekozen in de sectie **Model** van het bewerkingsformulier.

### De twee opties

- **GLM 5.1 (DeepInfra) - Slimmer, iets langzamer** - de standaardkeuze. Hogere redeneerkwaliteit, iets langzamer per aanroep. Aanbevolen voor moderatie-achtige agents (`Moderator` template, alles dat `ban_user` of `mark_comment_spam` aanroept) waar de kosten van een verkeerde beslissing hoog zijn.

- **GPT-OSS 120B Turbo (DeepInfra) - Sneller** - sneller per aanroep, lagere latentie. Aanbevolen voor agents met hoog volume en lage inzet (welkomstgroeter, thread-pinner) waar je reacties binnen enkele seconden wilt en de gevolgen van een misser klein zijn.

Beide modellen ondersteunen function calling, beide draaien via dezelfde OpenAI-compatibele API, en beide delen dezelfde per-tool schema's - dus je kunt een opgeslagen agent op elk moment tussen ze wisselen zonder andere configuratiewijzigingen.

### Kostenverschillen

De twee modellen hebben verschillende per-token kosten. De [budgetlimieten](#budgets-overview) van de agent worden uitgedrukt in de valuta van je account, niet in tokens, dus een wissel van het ene model naar het andere verandert hoeveel runs binnen je dagelijkse en maandelijkse limieten passen. De pagina [Run History](#run-history) toont de kosten per run in jouw valuta zodra een run is voltooid - een paar runs in de gaten houden na een wissel is de makkelijkste manier om het nieuwe verbruikstempo in te schatten.

### Tokens per run

Het tokenverbruik van de reactie van het model wordt bij elke trigger gelogd als **tokensUsed**. Het is opgenomen in de `trigger.succeeded` en `trigger.failed` webhook payloads (zie [Webhook Payloads](#webhook-payloads)) en wordt getoond in [Run Detail View](#run-detail-view). De hoeveelheid hangt af van:

- Hoeveel [Context](#context-options) je opneemt - thread-context, gebruikersgeschiedenis en paginametadata voegen allemaal tokens toe.
- Hoe lang je [Initiële prompt](#personality-prompt) en [Communityrichtlijnen](#community-guidelines) zijn.
- Hoeveel tools de agent in één run aanroept (elke tool-aanroep en het resultaat gaan heen en weer door het model).

**Max Tokens Per Trigger** (standaard 20.000) is de bovengrens per run, ingesteld per agent.

### Modellen wisselen

Je kunt op elk moment van model wisselen op het bewerkingsformulier. Bestaande run-geschiedenis en analytics behouden hun oorspronkelijke token- en kostennummers - deze worden vastgelegd op het moment van de run. Het nieuwe model geldt alleen voor runs die starten nadat je hebt opgeslagen.

Er is geen optie "gebruik welk model dan ook het goedkoopste is". De keuze is expliciet per agent.