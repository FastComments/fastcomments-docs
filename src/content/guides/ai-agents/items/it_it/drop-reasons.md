Quando un trigger viene attivato per un agente ma **non** comporta una chiamata LLM, la piattaforma registra uno "drop" con un motivo. I drop compaiono nella [pagina Analytics](#analytics-page) sotto "Trigger saltati (questo mese)".

### The full list of drop reasons

| Reason | What happened |
|---|---|
| `agentDaily` | È stato raggiunto il limite di budget giornaliero dell'agente. |
| `agentMonthly` | È stato raggiunto il limite di budget mensile dell'agente. |
| `tenantDaily` | È stato raggiunto il limite di budget giornaliero del tenant. |
| `tenantMonthly` | È stato raggiunto il limite di budget mensile del tenant. |
| `qps` | È stato raggiunto il limite di richieste per minuto dell'agente (finestra mobile di 60s). |
| `concurrency` | Il numero massimo di esecuzioni concorrenti dell'agente era già saturato. |

### What's not in this list

Un trigger che non raggiunge mai il percorso di dispatch non viene "dropped" con un motivo - semplicemente non viene dispatchato. Questo include:

- L'agente è **Disabilitato**.
- Il commento che ha attivato non corrisponde all'[ambito URL/locale](#scope-url-locale) dell'agente.
- L'azione che ha attivato è stata eseguita dallo stesso agente (prevenzione di loop).
- Il tenant ha fatturazione non valida.
- L'agente non è incluso nel piano del tenant.

Questi sono salti silenziosi, non drop. Non appaiono nel grafico dei drop su Analytics.

### Reading drops on Analytics

La [pagina Analytics](#analytics-page) mostra:

- **Trigger saltati (questo mese)** - conteggi raggruppati per motivo di drop.
- **Agenti al limite o vicini al limite** - suddivisione per agente di quali agenti stanno raggiungendo il limite, con un conteggio dei trigger scartati nel periodo corrente.

### What to do when you see drops

- **`agentDaily` / `agentMonthly`** - il limite dell'agente è troppo restrittivo. Aumentalo nel modulo di modifica o restringi l'ambito dell'agente (URL/locale, trigger più ristretti).
- **`tenantDaily` / `tenantMonthly`** - il limite a livello di account è troppo restrittivo. Aumentalo nelle impostazioni di fatturazione del tenant, oppure distribuisci la spesa su meno agenti.
- **`qps`** - il traffico sta raggiungendo il limite per minuto a finestra mobile. Spesso è un segno di un thread virale che genera trigger più velocemente di quanto l'agente possa eseguirli. I campi `maxTriggersPerMinute` e `maxConcurrent` dell'agente limitano questo; aumentarli incrementa il throughput ma aumenta anche il costo dei picchi.
- **`concurrency`** - stessa causa di fondo di `qps` ma relativa al conteggio in corso. Aumenta `maxConcurrent` se hai bisogno di più parallelismo.

### Drops vs errors

Un drop significa "il trigger non è mai stato eseguito". Un **errore** significa "il trigger è stato eseguito ma la chiamata LLM o il dispatch dello strumento è fallito". Gli errori sono tracciati separatamente nella pagina [Run History](#run-history) (stato `Error`).

### Drops can also stop replays

Gli stessi motivi di drop fermano i [test runs / replays](#test-runs-replays) in corso. La replay si ferma con stato Error e un messaggio che indica quale budget è stato raggiunto (per esempio, il budget giornaliero dell'agente).

### Loop prevention is silent on purpose

Non esiste un motivo di drop per "questo trigger proviene da un altro agente ed è stato saltato per prevenire un loop". Registrarlo appesantirebbe gli analytics senza fornire segnali utili - per progettazione, il fan-out degli agenti non dovrebbe mai causare un'esplosione di trigger. Se sospetti che un loop venga soppresso dove non dovrebbe, controlla i [Comment Logs](/guide-moderation.html#comment-logs) - il `botId` sui commenti creati dal bot è ciò su cui la verifica del loop si basa.

---