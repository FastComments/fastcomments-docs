**Refine Prompt** è il flusso di lavoro per modificare il [prompt iniziale](#personality-prompt) di un agente in risposta a decisioni specifiche con cui non sei d'accordo. Si avvia dalla [casella di approvazioni](#approval-workflow).

### Quando usarlo

Quando ti ritrovi a rifiutare lo stesso tipo di approvazione più e più volte - "l'agente continua a voler bannare persone per aver usato linguaggio forte senza un bersaglio" - il prompt dell'agente è la leva per correggere questo comportamento. Refine Prompt è un modo guidato per:

1. Scegliere un'approvazione specifica che rappresenta la decisione errata.
2. Modificare il prompt con il contesto completo di ciò che l'agente ha fatto e perché.
3. Salvare il nuovo prompt nell'agente.

Il risultato è un agente che, d'ora in poi, sarebbe improbabile che prenda la stessa decisione.

### Avvio del flusso

Dalla casella di approvazioni su `/auth/my-account/ai-agent-approvals`:

1. Apri un'approvazione **rifiutata**. La route rifiuta rigorosamente tutto ciò che non è REJECTED - le approvazioni in sospeso o con esecuzione fallita non sono idonee.
2. Clicca su **Refine prompt**.

Atterri sull'interfaccia di raffinamento del prompt su `/auth/my-account/ai-agent-approvals/:approvalId/refine-prompt`.

### Cosa mostra la pagina

- **L'approvazione** - il `toolName` e la `justification` dell'agente per la decisione rifiutata (la trascrizione completa dell'LLM non viene mostrata qui).
- **Il prompt attuale** - il [prompt iniziale](#personality-prompt) salvato dell'agente.
- **Un campo di feedback** - digiti un **feedback** che descrive cosa dovrebbe cambiare (fino a 2000 caratteri). L'LLM genera quindi il nuovo prompt proposto a partire dal tuo feedback.
- **Diff inline unificato** - un singolo diff inline tra il prompt attuale e quello proposto (rosso per le rimozioni, verde per le aggiunte).

Il contesto dell'approvazione rimane fissato in alto, così puoi continuare a fare riferimento a "il caso che sto correggendo" durante la modifica.

### Salvataggio

Il salvataggio aggiorna il campo `initialPrompt` dell'agente. Le esecuzioni passate (e le approvazioni passate) non vengono rieseguite retroattivamente - il nuovo prompt influisce solo sui trigger futuri. Se vuoi verificare che il nuovo prompt risolva il problema, esegui un [test run / replay](#test-runs-replays) sugli ultimi 7 giorni e controlla se il nuovo prompt produrrebbe comunque l'approvazione rifiutata.

### Cosa non fa questo flusso

- Non modifica le **community guidelines** - quel campo ha un proprio editor nel modulo principale di modifica dell'agente.
- Non modifica i **trigger**, gli **strumenti consentiti** o il **gating delle approvazioni** - questi rimangono nel modulo principale di modifica.
- Non versiona il prompt con rollback. Il prompt precedente non viene archiviato in una raccolta di cronologia separata. Se hai bisogno di fare rollback, copia il prompt attuale nel tuo sistema di tracciamento prima di modificarlo.

### Perché abbinare il raffinamento al replay

Modificare un prompt senza testare il risultato è un atto di fede. Il ciclo consigliato:

1. Rifiuta un'approvazione.
2. Raffina il prompt.
3. Esegui un [test run](#test-runs-replays) sugli ultimi 7 giorni.
4. Guarda la scheda **Deltas**. Il nuovo prompt ha spostato la decisione errata da "farebbe" a "non farebbe"? Ha spostato per errore anche decisioni corrette?
5. Itera.

Tre o quattro cicli di raffinamento + replay sono di solito sufficienti per ottenere un prompt stabile per un agente di moderazione.

### Alternativa: modifica diretta

Non sei obbligato a usare Refine Prompt - puoi anche semplicemente modificare l'agente dal modulo principale di modifica. L'unico vantaggio di Refine Prompt è che fissa un caso specifico di fallimento, così non perdi di vista ciò che stai correggendo.