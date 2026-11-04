Un agente ha uno di tre stati:

### Disabled

L'agente è disattivato. Nessun trigger viene elaborato e l'agente non appare nel percorso di dispatch. La sua cronologia di esecuzione, le analisi e la memoria rimangono - se lo riattivi in seguito, i dati storici sono ancora presenti.

Usa `Disabled` quando:
- Vuoi togliere un agente dalla rotazione senza perderlo.
- Un agente sta funzionando male e devi fermarlo immediatamente mentre indaghi.
- Stai ruotando gli agenti in modo stagionale (ad es. un salutatore solo per le festività).

### Dry Run - predefinito per i nuovi agenti

L'agente viene eseguito end-to-end - elabora i trigger, chiama l'LLM, seleziona le chiamate agli strumenti, calcola giustificazioni e confidenza - ma **nessuna azione reale viene eseguita**. Ogni esecuzione viene registrata con il badge **Dry Run** in [Run History](#run-history).

Usa `Dry Run` quando:
- Un nuovo agente è appena stato creato. Ogni template di partenza si trova in dry-run.
- Hai modificato il prompt o cambiato l'insieme di trigger e vuoi vedere come si comporta la modifica prima di confermarla.
- Stai eseguendo un [test run / replay](#test-runs-replays) (i replay forzano il dry-run indipendentemente dallo stato dell'agente).

La piattaforma addebita token per le esecuzioni in dry-run - la chiamata all'LLM avviene comunque, vengono solo saltati gli effetti collaterali. I limiti di budget si applicano anche al dry-run. Vedi [Budgets Overview](#budgets-overview).

### Enabled

L'agente esegue azioni reali. Le chiamate agli strumenti vengono eseguite - o messe in coda per [approvazione](#approval-workflow) se l'azione è soggetta a controllo.

Usa `Enabled` dopo che l'output del dry-run appare corretto.

### Cambio di stato

Puoi passare da uno stato all'altro nel modulo di modifica. Il passaggio da Dry Run a Enabled non riesegue retroattivamente le azioni del dry-run - quelle restano come cronologia del dry-run. I nuovi trigger da quel momento in poi vengono eseguiti in modalità live.

Il passaggio da Enabled a Disabled durante un'esecuzione **non** interrompe un'esecuzione in corso. Il trigger attualmente in esecuzione viene completato (con tutto ciò che ha già avviato); il trigger successivo viene scartato perché l'agente ora è Disabled.

### Stato durante problemi di fatturazione

Se la fatturazione del tuo tenant diventa non valida, tutti gli agenti vengono di fatto messi in pausa indipendentemente dallo stato salvato - i trigger vengono scartati con `BILLING_INVALID` finché la fatturazione non viene ripristinata. Il campo dello stato salvato non viene modificato; il dispatcher si limita a rifiutare l'esecuzione. Vedi [Plans and Eligibility](#plans-and-eligibility).