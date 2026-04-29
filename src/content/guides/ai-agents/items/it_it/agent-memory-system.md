Agent memory è una pool di coppie chiave-valore a livello di tenant, **condivisa**, che ogni agente del tuo tenant può leggere e scrivere. Esiste affinché gli agenti possano portare contesto tra esecuzioni.

### Why memory exists

Il contesto LLM è per-run. Senza memoria, un agente che emette un avvertimento a un utente non ha modo di conoscere quell'avvertimento la prossima volta che vede lo stesso utente. La policy di escalation della piattaforma - "warn before banning" - dipende dalla capacità dell'agente di trovare il precedente avvertimento. La memoria è ciò che rende questo possibile.

### Two kinds of memory

- **WARNING** - scritto automaticamente come parte del flusso di [`warn_user`](#tool-warn-user). L'agente non scrive manualmente record di tipo `WARNING`; sono un effetto collaterale dell'avvertire un utente.
- **NOTE** - scritto da [`save_memory`](#tools-overview). Contesto a scopo generale che l'agente vuole che futuri agenti conoscano.

La policy di escalation cerca specificamente record `WARNING` quando decide se un ban è giustificato.

### Tenant-scoped, agent-shared

Tutti gli agenti nel tuo tenant condividono **un'unica pool di memoria**. Una nota salvata dall'Agente A è visibile alle chiamate `search_memory` dell'Agente B. Questo è intenzionale: vuoi che le note di un agente di triage informino le decisioni di un agente moderatore.

`tenantId` viene impostato dall'esecutore dal tenant dell'agente stesso - mai dagli argomenti LLM - quindi perdite di memoria tra tenant sono impossibili per costruzione.

### What's in a memory record

Ogni voce di memoria contiene:

- **Quale agente l'ha scritta**, e quando.
- **Di chi tratta** - l'utente che questa memoria descrive. L'agente non può fabbricare questo; la piattaforma lo compila automaticamente da ciò che ha attivato l'agente.
- **Un segnale nascosto di alt-account** - la piattaforma registra inoltre (privatamente) l'impronta IP del commento originario, così ricerche di memoria future possono far emergere note riguardanti altri account che postano dallo stesso IP. L'impronta non viene mai mostrata all'agente o all'LLM.
- **La nota stessa** - fino a 2000 caratteri di testo libero.
- **Tag** per il recupero - fino a 10 tag brevi.
- **Un tipo** - o un avvertimento o una nota generale.
- **Un link opzionale al commento** - se la memoria è legata a un commento specifico.

### Search behavior

[`search_memory`](#tools-overview) restituisce fino a 25 record, ordinati dal più recente al meno recente, automaticamente limitati a (l'utente che ha attivato) O (altri account sull'IP che ha attivato). I risultati sono anche limitati a un totale di 8000 caratteri su tutto il contenuto restituito - le voci più vecchie vengono eliminate se si raggiunge il limite.

L'agente non passa `userId` o `targetIpHash`. Entrambi sono impostati dall'esecutore.

### Persistence

La memoria non ha **TTL**. I record persistono fino a quando non vengono rimossi esplicitamente. I record WARNING su un utente non vengono intenzionalmente mai cancellati automaticamente - la cronologia delle escalation deve essere rintracciabile indefinitamente o il controllo della piattaforma "search before banning" non avrebbe senso.

I tre modi in cui la memoria viene rimossa:

- Un moderatore elimina il commento sottostante - qualsiasi memoria legata a quel commento viene cancellata a cascata.
- Un utente viene eliminato - tutte le voci di memoria su quell'utente vengono rimosse nella stessa transazione.
- Il tuo tenant viene eliminato.

Attualmente non esiste un'interfaccia di amministrazione per eliminare singoli record di memoria.

### Memory in dry-run

Gli agenti in dry-run non scrivono memoria. Questo è voluto: le decisioni ipotetiche di un agente in dry-run non dovrebbero inquinare la pool di memoria condivisa. La lettura tramite `search_memory` funziona normalmente in dry-run - l'agente può vedere memorie reali da agenti live - semplicemente non può aggiungervi.

### Memory in replays

Stesso comportamento del dry-run: gli agenti di replay non scrivono memoria. I replay sono solo anteprime. Vedi [Test Runs (Replays)](#test-runs-replays).

### Constraints summary

| Limite | Valore |
|---|---|
| Lunghezza massima del contenuto della memoria | 2000 caratteri |
| Lunghezza massima del tag di memoria | 64 caratteri |
| Numero massimo di tag di memoria | 10 |
| Lunghezza massima della query di memoria | 200 caratteri |
| Limite risultati ricerca memoria | 25 record |
| Cap totale del contenuto nella ricerca memoria | 8000 caratteri |

### See also

- [Tool: save_memory](#tools-overview) for writing.
- [Tool: search_memory](#tools-overview) for reading.
- [Tool: warn_user](#tool-warn-user) - the only tool that writes WARNING-kind memory.
- [Tool: ban_user](#tool-ban-user) - the system prompt requires `search_memory` to be called before this.