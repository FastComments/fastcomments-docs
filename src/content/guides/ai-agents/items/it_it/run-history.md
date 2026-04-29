La Cronologia esecuzioni è il registro per agente di ogni trigger eseguito. Accessibile dalla pagina dell'elenco degli agenti tramite il pulsante **Esecuzioni**, o direttamente a `/auth/my-account/ai-agents/{agentId}/runs`.

### Cosa c'è nella pagina

Una tabella impaginata con una riga per ogni esecuzione:

| Column | Meaning |
|---|---|
| Date | Quando il trigger si è attivato (o quando il trigger differito è stato eseguito). |
| Status | **Iniziato**, **Successo**, o **Errore**. Un badge **Dry Run** viene mostrato accanto se l'esecuzione era in modalità di prova. |
| Cost | Costo per esecuzione nella valuta del tuo tenant. Vuoto per esecuzioni in corso (Iniziato). |
| Actions | Il numero di chiamate agli strumenti nell'esecuzione. |
| Details | Un pulsante **Visualizza** che apre [Vista dettaglio esecuzione](#run-detail-view). |

### Significato degli stati

- **Iniziato** - l'esecuzione è in corso, o è terminata prima di completare. Un'esecuzione bloccata su "Iniziato" per un tempo insolitamente lungo di solito indica un timeout della chiamata LLM.
- **Errore** - l'esecuzione è terminata ma ha fallito da qualche parte - la chiamata LLM ha restituito un errore, l'invio a uno strumento è fallito, ecc. La vista dettaglio contiene l'errore specifico.
- **Successo** - l'esecuzione è terminata senza errori. L'agente potrebbe non aver intrapreso azione, oppure nessuna, una o molte azioni.

### Stato vuoto

Quando un agente non ha esecuzioni, la pagina mostra: "Nessuna esecuzione per questo agente. Le esecuzioni abilitate appariranno qui una volta che un trigger si attiverà; usa Esegui test per visualizzare in anteprima cosa farebbe questo agente sui commenti passati."

Quell'ultima parte è intenzionale - il [flusso di esecuzione di test](#test-runs-replays) è il modo consigliato per popolare la Cronologia esecuzioni su un agente nuovo.

### Cosa non c'è nella pagina della cronologia esecuzioni

- **Trigger live che non sono mai stati inviati** - un trigger scartato a causa di budget, ambito o limitazioni di rate non appare in questa pagina. Quelli compaiono nella [pagina Analytics](#analytics-page) sotto "Trigger saltati".
- **Approvazioni** - le approvazioni in sospeso per le azioni intraprese in questa esecuzione risiedono nella [inbox delle approvazioni](#approval-workflow). L'azione appare nella vista dettaglio esecuzione come **In attesa di approvazione**.

### Conservazione

I singoli record delle esecuzioni sono conservati per 90 giorni, dopodiché l'esecuzione viene rimossa dalla cronologia. I costi e i conteggi dei trigger continuano ad essere aggregati nei riepiloghi analitici a lungo termine, quindi la [pagina Analytics](#analytics-page) mostra ancora i totali storici oltre quella finestra.

### Replay

Le esecuzioni prodotte dai Replay sono escluse dalla vista delle esecuzioni live per impostazione predefinita. La pagina [Esecuzioni di test (Replays)](#test-runs-replays) è dove è possibile visualizzarle.

### Filtraggio tra agenti

La tabella delle esecuzioni è per agente. Non esiste una vista delle esecuzioni cross-agente - la [pagina Analytics](#analytics-page) è il riepilogo cross-agente. Se hai bisogno di ispezionare esecuzioni su più agenti, gli eventi [Webhooks](#webhooks-overview) `trigger.succeeded` e `trigger.failed` sono quelli che dovresti inoltrare al tuo sistema.