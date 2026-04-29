Elke agent draait op een van twee LLM-modellen, gekozen in de **Model**-sectie van het bewerkingsformulier.

### De twee opties

- **GLM 5.1 (DeepInfra) - Smarter, bit slower** - de standaard. Hogere redeneerkwaliteit, iets langzamer per oproep. Aanbevolen voor moderatie-achtige agenten (`Moderator` template, anything that calls `ban_user` or `mark_comment_spam`) waar de kosten van een foutieve oproep hoog zijn.

- **GPT-OSS 120B Turbo (DeepInfra) - Faster** - sneller per oproep, lagere latentie. Aanbevolen voor agenten met een hoog volume en lage inzet (welkomstgroeter, draadvastzetter) waar je binnen enkele seconden reacties wilt en de gevolgen van een foutieve oproep klein zijn.

Beide modellen ondersteunen function calling, beide draaien via dezelfde OpenAI-compatibele API, en beide gebruiken dezelfde per-tool schemas - je kunt dus op elk gewenst moment een opgeslagen agent tussen hen wisselen zonder andere configuratiewijzigingen.

### Kostenverschillen

De twee modellen hebben verschillende kosten per token. De [budgetlimieten](#budgets-overview) van de agent worden uitgedrukt in de valuta van je account, niet in tokens, dus een switch van het ene model naar het andere verandert hoeveel runs binnen je dagelijkse en maandelijkse limieten passen. De [Rungeschiedenis](#run-history)-pagina toont de kosten per run in jouw valuta zodra een run is voltooid - een paar runs bekijken na een switch is de eenvoudigste manier om de nieuwe verbruikssnelheid in te schatten.

### Tokens per run

Het tokengebruik van het model voor de response wordt bij elke trigger gelogd als **tokensUsed**. Het is opgenomen in de `trigger.succeeded` en `trigger.failed` webhook-payloads (zie [Webhook Payloads](#webhook-payloads)) en wordt getoond in de [Run Detail View](#run-detail-view). De hoeveelheid hangt af van:

- Hoeveel [Context](#context-options) je opneemt - threadcontext, gebruikersgeschiedenis en paginametadata voegen allemaal tokens toe.
- Hoe lang je [Initiële prompt](#personality-prompt) en [Communityrichtlijnen](#community-guidelines) zijn.
- Hoeveel tools de agent in één run aanroept (elke toolaanroep en het resultaat maken een rondreis via het model).

**Max Tokens Per Trigger** (default 20,000) is de bovengrens per run, ingesteld per agent.

### Modellen wisselen

Je kunt op elk moment van model wisselen in het bewerkingsformulier. Bestaande runhistorie en analytics behouden hun originele token- en kostencijfers - die worden vastgelegd op het moment van de run. Het nieuwe model is alleen van toepassing op runs die starten nadat je hebt opgeslagen.

Er is geen optie "use whichever model is cheaper". De keuze is expliciet per agent.

---