Når en trigger udløses for en agent, men **ikke** resulterer i et LLM-opkald, registrerer platformen en "drop" med en årsag. Drops vises på [Analyser-siden](#analytics-page) under "Udløsere sprunget over (denne måned)".

### Den fulde liste over dropårsager

| Årsag | Hvad der skete |
|---|---|
| `agentDaily` | Agentens daglige budgetloft blev ramt. |
| `agentMonthly` | Agentens månedlige budgetloft blev ramt. |
| `tenantDaily` | Tenantens daglige budgetloft blev ramt. |
| `tenantMonthly` | Tenantens månedlige budgetloft blev ramt. |
| `qps` | Agentens per-minut-ratebegrænsning (rullende 60s-vindue) blev ramt. |
| `concurrency` | Agentens maksimale samtidige kørsler var allerede mættet. |

### Hvad er ikke på denne liste

En trigger, der aldrig når dispatch-stien, bliver ikke "droppet" med en årsag — den bliver simpelthen ikke afsendt. Det inkluderer:

- Agenten er **deaktiveret**.
- Den udløsende kommentar matcher ikke agentens [URL/lokale-omfang](#scope-url-locale).
- Den udløsende handling blev foretaget af den samme agent (loop-forebyggelse).
- Tenanten har ugyldig fakturering.
- Agenten er ikke i tenantens plan.

Dette er stille spring, ikke drops. De vises ikke i drops-diagrammet på Analyser-siden.

### Læsning af drops på Analyser-siden

På [Analyser-siden](#analytics-page) vises:

- **Udløsere sprunget over (denne måned)** - antal grupperet efter dropårsag.
- **Agenter ved eller tæt på deres loft** - per-agent opdeling af hvilke agenter, der presser loftet, med et antal droppede triggere i den aktuelle periode.

### Hvad du skal gøre, når du ser drops

- **`agentDaily` / `agentMonthly`** - agentens eget loft er for stramt. Enten forhøj loftet i redigeringsformularen eller indsnævr agentens omfang (URL/lokale, snævrere triggere).
- **`tenantDaily` / `tenantMonthly`** - konto-niveauets loft er for stramt. Forhøj det i tenantens faktureringsindstillinger, eller fordel forbruget på færre agenter.
- **`qps`** - trafikken rammer per-minut rullende-vindue-grænsen. Ofte et tegn på en viral tråd, der spreder triggere hurtigere, end agenten kan køre dem. Agentens `maxTriggersPerMinute` og `maxConcurrent` felter begrænser dette; at hæve dem øger gennemstrømningen, men øger også omkostninger ved spidsbelastning.
- **`concurrency`** - samme rodårsag som `qps`, men ved antallet under udførelse. Forhøj `maxConcurrent` hvis du har brug for mere parallelisme.

### Drops vs fejl

Et drop betyder "triggeren kørte aldrig". En **fejl** betyder "triggeren kørte, men LLM-opkaldet eller værktøjsafsendelsen mislykkedes". Fejl spores separat på [Kørselshistorik](#run-history)-siden (status `Error`).

### Drops kan også stoppe genafspilninger

De samme dropårsager stopper igangværende [testkørsler / genafspilninger](#test-runs-replays). Genafspilningen stopper med status Fejl og en besked, der angiver hvilket budget der blev ramt (for eksempel agentens daglige budget).

### Loop-forebyggelse er bevidst tavs

Der findes ingen drop-årsag for "denne trigger kom fra en anden agent og blev sprunget over for at forhindre et loop". At logge det ville fylde analyserne uden nyttigt signal — efter design bør agent-fan-out aldrig resultere i trigger-eksplosion. Hvis du mistænker, at et loop bliver undertrykt, hvor det ikke burde være, tjek [Kommentarlog](/guide-moderation.html#comment-logs) - `botId` på bot-forfattede kommentarer er det, som looptjekket bruger som nøgle.