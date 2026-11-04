Quando un trigger si attiva per un agente ma **non** risulta in una chiamata LLM, la piattaforma registra un "drop" con un motivo. I drop compaiono nella [Analytics page](#analytics-page) sotto "Triggers skipped (this month)".

### L'elenco completo dei motivi di drop

| Motivo | Cosa è successo |
|---|---|
| `agentDaily` | È stato raggiunto il limite giornaliero di budget dell'agente. |
| `agentMonthly` | È stato raggiunto il limite mensile di budget dell'agente. |
| `tenantDaily` | È stato raggiunto il limite giornaliero di budget del tenant. |
| `tenantMonthly` | È stato raggiunto il limite mensile di budget del tenant. |
| `qps` | È stato raggiunto il limite di frequenza al minuto dell'agente (finestra mobile di 60s). |
| `concurrency` | Il numero massimo di esecuzioni concorrenti dell'agente era già saturo. |

### Cosa non è in questo elenco

Un trigger che non raggiunge mai il percorso di dispatch non viene "droppato" con un motivo - semplicemente non viene inviato. Questo include:

- L'agente è **Disabled**.
- Il commento che ha attivato il trigger non corrisponde allo [scope URL/locale](#scope-url-locale) dell'agente.
- L'azione di attivazione è stata effettuata dallo stesso agente (prevenzione dei loop).
- Il tenant ha una fatturazione non valida.
- L'agente non è incluso nel piano del tenant.

Questi sono salti silenziosi, non drop. Non compaiono nel grafico dei drop su Analytics.

### Leggere i drop su Analytics

La [Analytics page](#analytics-page) mostra:

- **Triggers skipped (this month)** - conteggi raggruppati per motivo di drop.
- **Agents at or near their cap** - dettaglio per agente di quali agenti stanno raggiungendo il limite, con un conteggio dei trigger droppati nel periodo corrente.

### Cosa fare quando vedi drop

- **`agentDaily` / `agentMonthly`** - il limite dell'agente stesso è troppo stretto. Aumenta il limite nel modulo di modifica o restringi lo scope dell'agente (URL/locale, trigger più ristretti).
- **`tenantDaily` / `tenantMonthly`** - il limite a livello di account è troppo stretto. Aumentalo nelle impostazioni di fatturazione del tenant, o distribuisci la spesa su meno agenti.
- **`qps`** - il traffico sta raggiungendo il limite con finestra mobile al minuto. Spesso è segno di un thread virale che genera trigger più velocemente di quanto l'agente possa eseguirli. I campi `maxTriggersPerMinute` e `maxConcurrent` dell'agente limitano questo; aumentarli incrementa il throughput ma anche il costo di burst.
- **`concurrency`** - stessa causa principale di `qps` ma a livello di esecuzioni in corso. Aumenta `maxConcurrent` se ti serve più parallelismo.

### Drop vs errori

Un drop significa "il trigger non è mai stato eseguito". Un **errore** significa "il trigger è stato eseguito ma la chiamata LLM o il dispatch dello strumento è fallito". Gli errori sono tracciati separatamente nella pagina [Run History](#run-history) (stato `Error`).

### I drop possono interrompere anche i replay

Gli stessi motivi di drop interrompono le [esecuzioni di test / replay](#test-runs-replays) in corso. Il replay si interrompe con stato Error e un messaggio che indica quale budget è stato raggiunto (ad esempio, il budget giornaliero dell'agente).

### La prevenzione dei loop è silenziosa di proposito

Non esiste un motivo di drop per "questo trigger proviene da un altro agente ed è stato saltato per prevenire un loop". Registrare il tutto ingombrerebbe le analisi senza alcun segnale utile - per design, il fan-out degli agenti non dovrebbe mai risultare in un'esplosione di trigger. Se sospetti che un loop venga soppresso quando non dovrebbe esserlo, controlla i [Comment Logs](/guide-moderation.html#comment-logs) - il `botId` sui commenti scritti dai bot è ciò su cui si basa il controllo dei loop.