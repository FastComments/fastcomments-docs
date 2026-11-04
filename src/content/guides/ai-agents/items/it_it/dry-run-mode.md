**Dry Run** è la modalità di sicurezza con cui ogni nuovo agente parte. L'agente viene eseguito end-to-end tranne la parte in cui tocca la tua community.

### Cosa viene eseguito in Dry Run

- I trigger si attivano normalmente.
- Il prompt dell'agente, le [community guidelines](#community-guidelines) e il [contesto](#context-options) vengono assemblati.
- L'LLM viene chiamato.
- Il modello sceglie le tool call e fornisce giustificazioni + punteggi di confidenza.
- L'esecuzione viene registrata con un badge **Dry Run** così da essere chiaramente distinta dalle esecuzioni live.

### Cosa non viene eseguito in Dry Run

- Nessun commento viene pubblicato, nessun voto viene espresso, nessun commento viene fissato/defissato/bloccato/sbloccato.
- Nessun commento viene contrassegnato come spam, approvato o revisionato.
- Nessun utente viene bannato, avvertito o gli viene assegnato un badge.
- Nessuna email viene inviata.
- Nessuna memoria viene scritta. (Sì - inclusa la memoria. Gli agenti in dry-run non possono inquinare il pool di memoria condiviso con decisioni ipotetiche.)
- Nessun webhook viene attivato per le azioni dei tool. (I webhook a livello di trigger `trigger.succeeded` / `trigger.failed` si attivano comunque e il payload include `wasDryRun: true`. Vedi [Payload dei Webhook](#webhook-payloads).)

### Quanto costa

Le esecuzioni in Dry Run effettuano **la stessa chiamata LLM** che farebbe un'esecuzione Enabled. I token vengono addebitati, si applicano i [limiti di budget](#budgets-overview) e le esecuzioni conteggiano ai fini dei limiti giornalieri/mensili per agente e per tenant.

Quel costo è il prezzo di un'anteprima fedele. Una modalità "salta la chiamata LLM" non ti darebbe alcun segnale su come si comporterebbe l'agente.

### Leggere i risultati del dry-run

In [Run History](#run-history), le esecuzioni in dry-run sono contrassegnate con il badge **Dry Run** nella colonna di stato. Le azioni all'interno di ogni esecuzione appaiono identiche alle azioni live - stesso nome di tool, stessi argomenti, stessa giustificazione e confidenza - solo che nessuna di esse si è realmente verificata.

La [pagina Analytics](#analytics-page) scompone le esecuzioni "dry-run vs live" per mese, così puoi vedere quanto della tua spesa in token è andata all'osservazione.

### Uscire dalla modalità Dry Run

Modifica l'agente e cambia **Status** in **Enabled**. Il prossimo trigger verrà eseguito live.

Puoi anche fare il contrario - da Enabled di nuovo a Dry Run - se l'agente inizia a fare cose che non ti piacciono. Non c'è alcuna penalità.

### I replay forzano la modalità Dry Run

La funzionalità [Test Runs (Replays)](#test-runs-replays) esegue l'agente sui commenti storici **sempre in dry-run**, indipendentemente dallo stato salvato dell'agente. I replay non possono compiere azioni reali sui commenti passati. Questo è voluto - il replay è uno strumento di anteprima, non uno strumento di moderazione.