La memoria degli agent è un pool chiave-valore con ambito tenant, **condiviso**, da cui ogni agent nel tuo tenant può leggere e scrivere. Esiste affinché gli agent possano mantenere il contesto tra le diverse esecuzioni.

### Perché esiste la memoria

Il contesto LLM è per singola esecuzione. Senza memoria, un agent che emette un avvertimento a un utente non ha modo di conoscere tale avvertimento la volta successiva in cui incontra lo stesso utente. La policy di escalation della piattaforma - "avvertire prima di bannare" - dipende dalla capacità dell'agent di trovare l'avvertimento precedente. La memoria è ciò che rende possibile questo.

### Due tipi di memoria

- **WARNING** - scritto automaticamente come parte del flusso [`warn_user`](#tool-warn-user). L'agent non scrive i record `WARNING` a mano; sono un effetto collaterale dell'avvertimento di un utente.
- **NOTE** - scritto da [`save_memory`](#tools-overview). Contesto generico che l'agent vuole che i futuri agent conoscano.

La policy di escalation cerca specificamente i record `WARNING` quando decide se un ban è giustificato.

### Con ambito tenant, condivisa tra gli agent

Tutti gli agent nel tuo tenant condividono **un unico pool di memoria**. Una nota salvata dall'Agent A è visibile alle chiamate `search_memory` dell'Agent B. Questo è intenzionale - vuoi che le note di un agent di triage informino le decisioni di un agent moderatore.

`tenantId` è impostato dall'executor in base al tenant dell'agent stesso - mai dagli argomenti LLM - quindi le fughe di memoria tra tenant sono impossibili per costruzione.

### Cosa contiene un record di memoria

Ogni voce di memoria contiene:

- **Quale agent l'ha scritta**, e quando.
- **A chi si riferisce** - l'utente descritto da questa memoria. L'agent non può fabbricarlo; la piattaforma lo compila automaticamente in base a ciò che ha attivato l'agent.
- **Un segnale nascosto di account alternativi** - la piattaforma registra anche (privatamente) l'impronta IP del commento di origine, così le future ricerche in memoria possono far emergere note su altri account che pubblicano dallo stesso IP. L'impronta non viene mai mostrata all'agent o all'LLM.
- **La nota stessa** - fino a 2000 caratteri di testo libero.
- **Tag** per il recupero - fino a 10 tag brevi.
- **Un tipo** - o un avvertimento o una nota generica.
- **Un link opzionale a un commento** - se la memoria è legata a un commento specifico.

### Comportamento di ricerca

[`search_memory`](#tools-overview) restituisce fino a 25 record, ordinati dal più recente, con ambito automaticamente limitato a (l'utente del trigger) OPPURE (altri account sull'IP del trigger). I risultati sono anche limitati a 8000 caratteri totali per tutto il contenuto restituito - le voci più vecchie vengono scartate se il limite viene raggiunto.

L'agent non passa `userId` né `targetIpHash`. Entrambi sono impostati dall'executor.

### Persistenza

La memoria **non ha TTL**. I record persistono finché non vengono rimossi esplicitamente. I record WARNING su un utente non vengono mai eliminati automaticamente intenzionalmente - la cronologia di escalation deve essere ritrovabile indefinitamente, altrimenti il controllo "cerca prima di bannare" della piattaforma non ha senso.

I tre modi in cui la memoria viene rimossa:

- Un moderatore elimina il commento sottostante - qualsiasi memoria legata a quel commento viene rimossa a cascata.
- Un utente viene eliminato - tutte le voci di memoria su quell'utente vengono rimosse nella stessa transazione.
- Il tuo tenant viene eliminato.

Al momento non esiste un'interfaccia di amministrazione per eliminare singoli record di memoria.

### Memoria in dry-run

Gli agent in dry-run **non** scrivono memoria. Questo è voluto: le decisioni ipotetiche di un agent in dry-run non dovrebbero inquinare il pool di memoria condiviso. La lettura tramite `search_memory` funziona normalmente in dry-run - l'agent può vedere le memorie reali degli agent attivi - semplicemente non può aggiungerne.

### Memoria nei replay

Come per il dry-run: gli agent di replay non scrivono memoria. I replay sono solo anteprime. Vedi [Test Runs (Replays)](#test-runs-replays).

### Riepilogo dei vincoli

| Limite | Valore |
|---|---|
| Lunghezza massima del contenuto della memoria | 2000 caratteri |
| Lunghezza massima del tag della memoria | 64 caratteri |
| Numero massimo di tag della memoria | 10 |
| Lunghezza massima della query di memoria | 200 caratteri |
| Limite di risultati della ricerca in memoria | 25 record |
| Limite totale di contenuto della ricerca in memoria | 8000 caratteri |

### Vedi anche

- [Tool: save_memory](#tools-overview) per la scrittura.
- [Tool: search_memory](#tools-overview) per la lettura.
- [Tool: warn_user](#tool-warn-user) - l'unico tool che scrive memoria di tipo WARNING.
- [Tool: ban_user](#tool-ban-user) - il prompt di sistema richiede che `search_memory` venga chiamato prima di questo.