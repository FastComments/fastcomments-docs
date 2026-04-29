**Raffina il prompt** è il flusso di lavoro per modificare il [prompt iniziale](#personality-prompt) di un agente in risposta a decisioni specifiche con cui non sei d'accordo. Viene avviato dalla [casella delle approvazioni](#approval-workflow).

### When to use it

Quando ti trovi a rifiutare lo stesso tipo di approvazione più e più volte - "l'agente continua a voler bannare persone per l'uso di linguaggio forte senza un bersaglio" - il prompt dell'agente è la leva per risolvere ciò. Raffina il prompt è un modo guidato per:

1. Selezionare una specifica approvazione che rappresenta la decisione sbagliata.
2. Modificare il prompt con il contesto completo di ciò che l'agente ha fatto e perché.
3. Salvare il nuovo prompt sull'agente.

Il risultato è un agente che, in futuro, sarà improbabile che prenda la stessa decisione.

### Launching the flow

Dalla casella delle approvazioni su `/auth/my-account/ai-agent-approvals`:

1. Apri una approvazione **rejected**. La route rigetta categoricamente tutto tranne REJECTED - le approvazioni pending e execution-failed non sono idonee.
2. Clicca **Raffina il prompt**.

Arriverai all'interfaccia di refine del prompt su `/auth/my-account/ai-agent-approvals/:approvalId/refine-prompt`.

### What the page shows

- **The approval** - il `toolName` dell'agente e la sua `justification` per la decisione rifiutata (la trascrizione completa del LLM non è mostrata qui).
- **The current prompt** - il [prompt iniziale](#personality-prompt) salvato dall'agente.
- **A feedback input** - inserisci un **feedback** che descriva cosa dovrebbe cambiare (fino a 2000 caratteri). L'LLM genera quindi il nuovo prompt proposto a partire dal tuo feedback.
- **Unified inline diff** - un unico diff inline tra il prompt corrente e quello proposto (rosso per rimosso, verde per aggiunto).

Il contesto dell'approvazione resta fissato in alto così puoi continuare a fare riferimento al "caso che sto correggendo" mentre modifichi.

### Save

Il salvataggio aggiorna il campo `initialPrompt` dell'agente. Esecuzioni passate (e approvazioni passate) non vengono rieseguite retroattivamente - il nuovo prompt influenza solo i trigger futuri. Se vuoi verificare che il nuovo prompt risolva il problema, esegui un [test run / replay](#test-runs-replays) sugli ultimi 7 giorni e verifica se il nuovo prompt produrrebbe ancora l'approvazione rifiutata.

### What the flow does not do

- Non modifica le **community guidelines** - quel campo ha un proprio editor nel modulo principale di modifica dell'agente.
- Non modifica **triggers**, **allowed tools**, o **approval gating** - questi rimangono nel modulo principale di modifica.
- Non versiona il prompt con rollback. Il prompt precedente non è memorizzato in una raccolta di cronologia separata. Se devi tornare indietro, copia il prompt corrente nel tuo sistema di tracciamento prima di modificarlo.

### Why pair refine with replay

Modificare un prompt senza testarne il risultato è una pratica basata sulla fede. Il ciclo raccomandato:

1. Rifiuta un'approvazione.
2. Raffina il prompt.
3. Esegui un [test run](#test-runs-replays) sugli ultimi 7 giorni.
4. Guarda la scheda **Deltas**. Il nuovo prompt ha spostato la decisione sbagliata da "would do" a "would not do"? Ha spostato inavvertitamente anche decisioni corrette?
5. Itera.

Tre o quattro cicli di raffinamento + replay sono di solito sufficienti per ottenere un prompt stabile per un agente di moderazione.

### Direct edit alternative

Non sei obbligato a usare Raffina il prompt - puoi anche semplicemente modificare l'agente dal modulo principale di modifica. L'unico vantaggio di Raffina il prompt è che fissa un caso specifico che fallisce così non perdi di vista ciò che stai cercando di correggere.