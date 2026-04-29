**Dry Run** è la modalità di sicurezza in cui ogni nuovo agente inizia. L'agente esegue l'intero flusso, eccetto la parte in cui interagisce con la tua community.

### What runs in Dry Run

- I trigger si attivano normalmente.
- Il prompt dell'agente, le [community guidelines](#community-guidelines) e il [context](#context-options) vengono assemblati.
- Viene chiamato l'LLM.
- Il modello sceglie le chiamate agli strumenti e fornisce giustificazioni + punteggi di confidenza.
- L'esecuzione viene registrata con un badge **Dry Run**, in modo da distinguerla chiaramente dalle esecuzioni live.

### What does not run in Dry Run

- Nessun commento viene pubblicato, nessun voto viene espresso, nessun commento viene fissato/non fissato/bloccato/sbloccato.
- Nessun commento viene contrassegnato come spam, approvato o revisionato.
- Nessun utente viene bannato, avvertito o premiato con un badge.
- Nessuna email viene inviata.
- Nessuna memoria viene scritta. (Sì — compresa la memoria. Gli agenti in dry-run non possono avvelenare il pool di memoria condivisa con decisioni ipotetiche.)
- Nessun webhook viene attivato per azioni degli strumenti. (I webhook a livello di trigger `trigger.succeeded` / `trigger.failed` vengono comunque attivati e il payload include `wasDryRun: true`. Vedi [Webhook Payloads](#webhook-payloads).)

### What it costs

Dry Run esegue **la stessa chiamata LLM** che verrebbe fatta in un'esecuzione Enabled. I token vengono addebitati, si applicano i [budget caps](#budgets-overview) e le esecuzioni vengono conteggiate nei limiti giornalieri/mensili per agente e per tenant.

Questo costo è il prezzo per ottenere un'anteprima fedele. Una modalità che "salta la chiamata LLM" non ti fornirebbe alcun segnale su come si comporterebbe l'agente.

### Reading dry-run results

In [Run History](#run-history), le esecuzioni dry-run sono contrassegnate con il badge **Dry Run** nella colonna di stato. Le azioni all'interno di ogni esecuzione appaiono identiche alle azioni live - stesso nome dello strumento, stessi argomenti, stessa giustificazione e stessa confidenza - eccetto che nessuna di esse è avvenuta.

La [Analytics page](#analytics-page) suddivide le esecuzioni "dry-run vs live" per mese in modo da poter vedere quanto del tuo consumo di token è stato destinato all'osservazione.

### Switching out of Dry Run

Modifica l'agente e imposta **Status** su **Enabled**. Il prossimo trigger verrà eseguito live.

Puoi anche procedere nella direzione opposta - da Enabled a Dry Run - se l'agente inizia a fare cose che non ti piacciono. Non c'è alcuna penalità.

### Replays force Dry Run

La funzionalità [Test Runs (Replays)](#test-runs-replays) esegue l'agente contro commenti storici **sempre in dry-run**, indipendentemente dallo stato salvato dell'agente. I replay non possono compiere azioni reali sui commenti passati. Questo è voluto - il replay è uno strumento di anteprima, non uno strumento di moderazione.

---