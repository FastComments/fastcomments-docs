Wanneer een trigger afgaat voor een agent maar **niet** resulteert in een LLM-aanroep, registreert het platform een "drop" met een reden. Drops verschijnen in de [Analytics-pagina](#analytics-page) onder "Triggers overgeslagen (deze maand)".

### De volledige lijst met dropredenen

| Reden | Wat er gebeurde |
|---|---|
| `agentDaily` | Het dagelijkse budgetplafond van de agent werd bereikt. |
| `agentMonthly` | Het maandelijkse budgetplafond van de agent werd bereikt. |
| `tenantDaily` | Het dagelijkse budgetplafond van de tenant werd bereikt. |
| `tenantMonthly` | Het maandelijkse budgetplafond van de tenant werd bereikt. |
| `qps` | De per-minuut rollend-vensterlimiet van de agent (60s rollend venster) werd bereikt. |
| `concurrency` | Het maximaal aantal gelijktijdige runs van de agent was al verzadigd. |

### Wat staat niet in deze lijst

Een trigger die nooit het dispatchpad bereikt wordt niet "gedropt" met een reden - deze wordt gewoon niet gedispatched. Dat omvat:

- Agent is **Uitgeschakeld**.
- De triggerende opmerking komt niet overeen met het [URL/locale-bereik](#scope-url-locale) van de agent.
- De triggerende actie werd uitgevoerd door dezelfde agent (luspreventie).
- De tenant heeft ongeldige facturering.
- De agent zit niet in het plan van de tenant.

Dit zijn stille overslagen, geen drops. Ze verschijnen niet in de dropsgrafiek op Analytics.

### Drops lezen op Analytics

De [Analytics-pagina](#analytics-page) toont:

- **Triggers overgeslagen (deze maand)** - aantallen gegroepeerd op dropreden.
- **Agents bij of in de buurt van hun limiet** - per-agent uitsplitsing welke agents de limiet benaderen, met een telling van gedropte triggers in de huidige periode.

### Wat te doen wanneer je drops ziet

- **`agentDaily` / `agentMonthly`** - de eigen limiet van de agent is te krap. Verhoog de limiet op het bewerkingsformulier of beperk de scope van de agent (URL/locale, nauwkeurigere triggers).
- **`tenantDaily` / `tenantMonthly`** - de accountniveaulimiet is te krap. Verhoog deze in de tenant-facturatie-instellingen, of verdeel het verbruik over minder agents.
- **`qps`** - het verkeer raakt de per-minuut rollend-vensterlimiet. Vaak een teken van een viraal draadje dat triggers sneller verspreidt dan de agent ze kan uitvoeren. De velden `maxTriggersPerMinute` en `maxConcurrent` van de agent begrenzen dit; het verhogen ervan vergroot de doorvoersnelheid maar verhoogt ook de piekkosten.
- **`concurrency`** - dezelfde onderliggende oorzaak als `qps` maar dan bij het aantal in uitvoering. Verhoog `maxConcurrent` als je meer parallelisme nodig hebt.

### Drops versus fouten

Een drop is "de trigger heeft nooit gedraaid". Een **fout** is "de trigger draaide wel, maar de LLM-aanroep of tool-dispatch is mislukt". Fouten worden apart bijgehouden op de [Uitvoeringsgeschiedenis](#run-history) pagina (status `Error`).

### Drops kunnen ook herhalingen stoppen

Dezelfde dropredenen stoppen lopende [testuitvoeringen / herhalingen](#test-runs-replays). De herhaling stopt met een Error-status en een bericht dat aangeeft welk budget werd geraakt (bijvoorbeeld het dagelijkse budget van de agent).

### Luspreventie is opzettelijk stil

Er is geen dropreden voor "deze trigger kwam van een andere agent en werd overgeslagen om een lus te voorkomen". Dit registreren zou de analytics vervuilen zonder nuttige informatie — per ontwerp mag agent-fan-out nooit leiden tot trigger-explosie. Als je vermoedt dat een lus wordt onderdrukt waar dat niet zou moeten, controleer dan de [Opmerkinglogboeken](/guide-moderation.html#comment-logs) - de `botId` op door bots geschreven opmerkingen is waar de luscontrole op baseert.