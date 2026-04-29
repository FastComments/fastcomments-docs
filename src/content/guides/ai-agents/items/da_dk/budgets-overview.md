Hver agent har forbrugsgrænser. Platformen stopper med at udløse agenten, når en grænse nås, og genoptager, når perioden ruller over.

### To omfang, to perioder

Der er i alt fire grænser - to omfang (per-agent, per-tenant) ganget med to perioder (dagligt, månedligt).

| Scope | Period | Where you set it |
|---|---|---|
| Per-agent daily | UTC day | Agent edit form -> **Budget** -> **Daily budget** |
| Per-agent monthly | calendar month | Agent edit form -> **Budget** -> **Monthly budget** |
| Per-tenant daily | UTC day | Plan-derived (no separate user-facing input) |
| Per-tenant monthly | calendar month | Plan-derived (no separate user-facing input) |

En udløser afvikles kun, hvis **alle fire grænser** tillader det. Den første grænse, der opbruges, er den, der forårsager, at udløseren ikke kører.

### Valuta

Per-agent budgetter indtastes i din konto-valuta.

### Hvad sker der, når en grænse nås

- Udløseren registreres som **dropped** med en [drop reason](#drop-reasons) som `agentDaily` eller `tenantMonthly`.
- Antallet af droppede forekomster vises på [Analytics page](#analytics-page) under "Triggers skipped (this month)".
- Der foretages ikke et LLM-kald; der bruges ingen tokens på den droppede udløser.
- Agentens status ændres ikke - den kan blot ikke afvikles, før perioden ruller over.

### Periode-rollover

- **Daglige** grænser nulstilles ved midnat UTC.
- **Månedlige** grænser nulstilles ved starten af hver kalender måned, UTC.

Der er ingen overførsel af ubrugt budget til næste periode.

### Hård grænse vs bløde advarsler

Grænser er **hårde**. Der er ingen "overskrid med 10% med advarsel"-tilstand. Når grænsen nås, stopper afviklingen.

Den "bløde" del er [Budget Alerts](#budget-alerts)-e-mailsene - du modtager en e-mail ved konfigurerbare tærskler (standard 80% og 100%), så du kan hæve grænsen, før trafikken begynder at blive afskåret.

### Hvor du kan se aktuelt forbrug

- [Analytics page](#analytics-page) - per-agent og tenant-dækkende budgetforbrug med grænsemarkører.
- Agentens redigeringsformulars **Stats**-sektion.
- Listevisningen (antal ventende godkendelser og nylige kørsler vises på agentkortet).

### Valg af budget

Nogle tommelfingerregler:

- **En ny agent** - fastsæt et budget. Overvåg [Run History](#run-history) i en uge. Juster baseret på observeret omkostning per kørsel × forventet trigger-volumen.
- **En agent med høj volumen** (f.eks. new-comment trigger på et travlt site) - den daglige grænse er den, der fanger en løbsk loop. Vælg en daglig grænse der er 2–3× dit forventede daglige forbrug, så en normalt travl dag passer komfortabelt under den.
- **En summarizer eller agent med tung kontekst** - omkostning per kørsel er høj. Sæt en strammere daglig grænse for at forhindre, at en dårlig dag sprænger månedens budget.

### Budget-omgåelse for afspilninger

[Test runs / replays](#test-runs-replays) er underlagt deres **egne** hårde grænse (sat på replay-formularen, separat fra agentens daglige/månedlige grænser), OG agent- og tenant-grænserne. Den først nåede grænse stopper afspilningen.

### Se også

- [Budget Alerts](#budget-alerts) for e-mail-notifikationerne.
- [Cost Model](#cost-model) for hvordan platformen konverterer tokens til dollars.
- [Drop Reasons](#drop-reasons) for den komplette liste over årsager til, at en udløser ikke kører.