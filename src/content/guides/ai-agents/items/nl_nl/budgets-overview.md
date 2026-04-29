Elke agent heeft uitgavelimieten. Het platform stopt met dispatchen van de agent wanneer een limiet is bereikt en hervat zodra de periode vervalt.

### Twee scopes, twee perioden

Er zijn in totaal vier limieten - twee scopes (per-agent, per-tenant) gecombineerd met twee perioden (dagelijks, maandelijks).

| Scope | Periode | Waar je het instelt |
|---|---|---|
| Per-agent dagelijks | UTC-dag | Agent bewerkingsformulier -> **Budget** -> **Dagelijks budget** |
| Per-agent maandelijks | kalendermaand | Agent bewerkingsformulier -> **Budget** -> **Maandelijks budget** |
| Per-tenant dagelijks | UTC-dag | Afgeleid van het abonnement (geen aparte gebruikersinvoer) |
| Per-tenant maandelijks | kalendermaand | Afgeleid van het abonnement (geen aparte gebruikersinvoer) |

Een trigger wordt alleen uitgevoerd als **alle vier limieten** het toestaan. De eerste limiet die uitgeput raakt is degene die de trigger stopt.

### Valuta

Per-agent budgetten worden ingevoerd in de valuta van je account.

### Wat er gebeurt wanneer een limiet wordt bereikt

- De trigger wordt geregistreerd als **gedropped** met een [drop reason](#drop-reasons) zoals `agentDaily` of `tenantMonthly`.
- Het aantal drops verschijnt op de [Analysepagina](#analytics-page) onder "Triggers skipped (this month)".
- Er wordt geen LLM-aanroep gedaan; er worden geen tokens gespendeerd aan de gedropte trigger zelf.
- De status van de agent blijft ongewijzigd - hij kan alleen geen uitvoering starten totdat de periode is verstreken.

### Periodeverval

- **Dagelijkse** limieten worden gereset om middernacht UTC.
- **Maandelijkse** limieten worden gereset aan het begin van elke kalendermaand, UTC.

Er is geen overdracht van ongebruikt budget naar de volgende periode.

### Harde limiet vs zachte waarschuwingen

Limieten zijn **hard**. Er is geen modus om "met 10% te overschrijden met waarschuwing". Wanneer de limiet is bereikt, stopt het dispatchen.

Het "zachte" deel zijn de e-mails van de [Budgetwaarschuwingen](#budget-alerts) - je ontvangt een e-mail bij configureerbare drempels (standaard 80% en 100%) zodat je de limiet kunt verhogen voordat het verkeer begint af te nemen.

### Waar je huidig gebruik kunt lezen

- [Analysepagina](#analytics-page) - per-agent en tenant-breed budgetgebruik met limietmarkeringen.
- De **Stats** sectie van het agent bewerkingsformulier.
- De lijstweergave (aantal openstaande goedkeuringen en recente runs staat op de agentkaart).

### Een budget kiezen

Enkele vuistregels:

- **Een nieuwe agent** - bepaal een budget. Kijk een week naar de [Uitvoeringsgeschiedenis](#run-history). Pas aan op basis van waargenomen kosten per run × verwacht triggervolume.
- **Een agent met veel verkeer** (bijv. new-comment trigger op een drukke site) - de dagelijkse limiet is wat een runaway loop opvangt. Kies een dagelijkse limiet die 2–3x je verwachte dagelijkse uitgaven is zodat een normale drukke dag comfortabel binnen blijft.
- **Een summarizer of context-zware agent** - kosten per run zijn hoog. Stel een strakker dagelijks budget in om te voorkomen dat een slechte dag de maandelijkse limiet doorbreekt.

### Budgetomzeiling voor replays

[Testruns / herhalingen](#test-runs-replays) vallen onder hun **eigen** harde limiet (ingesteld op het replay-formulier, los van de dagelijkse/maandelijkse limieten van de agent), EN onder de agent- en tenant-limieten. Welke limiet ook het eerst wordt bereikt stopt de replay.

### Zie ook

- [Budgetwaarschuwingen](#budget-alerts) voor de e-mailmeldingen.
- [Kostmodel](#cost-model) voor hoe het platform tokens naar dollars converteert.
- [Redenen voor uitval](#drop-reasons) voor de volledige lijst met redenen waarom een trigger niet draait.