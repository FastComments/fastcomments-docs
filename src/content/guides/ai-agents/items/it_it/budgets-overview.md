Ogni agente ha dei limiti di spesa. La piattaforma smette di inviare (dispatch) l'agente quando viene raggiunto un limite e riprende una volta che il periodo viene azzerato.

### Due ambiti, due periodi

Ci sono quattro limiti in totale - due ambiti (per-agente, per-tenant) incrociati con due periodi (giornaliero, mensile).

| Ambito | Periodo | Dove impostarlo |
|---|---|---|
| Per-agent daily | UTC day | Agent edit form -> **Budget** -> **Daily budget** |
| Per-agent monthly | calendar month | Agent edit form -> **Budget** -> **Monthly budget** |
| Per-tenant daily | UTC day | Plan-derived (no separate user-facing input) |
| Per-tenant monthly | calendar month | Plan-derived (no separate user-facing input) |

Un trigger viene inviato solo se **tutti e quattro i limiti** lo consentono. Il primo limite ad esaurirsi è quello che fa scattare lo scarto del trigger.

### Valuta

I budget per agente vengono inseriti nella valuta del tuo account.

### Cosa succede quando viene raggiunto un limite

- Il trigger viene registrato come **dropped** con un [drop reason](#drop-reasons) come `agentDaily` o `tenantMonthly`.
- Il conteggio degli scarti appare nella [Pagina Analytics](#analytics-page) sotto "Trigger saltati (questo mese)".
- Non viene effettuata alcuna chiamata LLM; non vengono spesi token per il trigger scartato.
- Lo stato dell'agente rimane invariato: semplicemente non può essere attivato finché il periodo non viene azzerato.

### Azzeramento dei periodi

- I limiti **giornalieri** vengono azzerati a mezzanotte UTC.
- I limiti **mensili** vengono azzerati all'inizio di ogni mese del calendario, UTC.

Non c'è trasferimento del budget non utilizzato al periodo successivo.

### Hard cap vs avvisi soft

I limiti sono **rigidi**. Non esiste una modalità "supera del 10% con avviso". Quando il limite viene raggiunto, le dispatch si fermano.

La parte "soft" sono le email di [Budget Alerts](#budget-alerts) - ricevi una email a soglie configurabili (predefinite 80% e 100%) così puoi aumentare il limite prima che il traffico inizi a calare.

### Dove leggere l'utilizzo attuale

- [Pagina Analytics](#analytics-page) - utilizzo del budget per agente e a livello tenant con indicatori dei limiti.
- La sezione **Stats** nel modulo di modifica dell'agente.
- La vista elenco (conteggio delle approvazioni in sospeso e esecuzioni recenti è sulla scheda dell'agente).

### Scegliere un budget

Alcune regole pratiche:

- **Un nuovo agente** - determinare il budget. Osserva la [Cronologia esecuzioni](#run-history) per una settimana. Regola in base al costo osservato per esecuzione × volume di trigger previsto.
- **Un agente ad alto volume** (ad es., trigger new-comment su un sito molto trafficato) - il limite giornaliero è ciò che cattura un loop incontrollato. Scegli un limite giornaliero che sia 2-3x della tua spesa giornaliera prevista così da permettere a una giornata normale e intensa di rimanere comodamente al di sotto.
- **Un agente riassuntore o che richiede molto contesto** - il costo per esecuzione è elevato. Imposta un limite giornaliero più stretto per evitare che una giornata negativa eroda il mensile.

### Bypass del budget per i replay

[Esecuzioni di prova / replay](#test-runs-replays) sono soggette al loro **proprio** limite rigido (impostato nel modulo del replay, separato dai limiti giornalieri/mensili dell'agente), E inoltre sono soggette ai limiti dell'agente e del tenant. Qualunque venga raggiunto per primo ferma il replay.

### Vedi anche

- [Budget Alerts](#budget-alerts) per le notifiche email.
- [Cost Model](#cost-model) per come la piattaforma converte i token in dollari.
- [Drop Reasons](#drop-reasons) per l'elenco completo dei motivi per cui un trigger non viene eseguito.

---