---
Il costo dell'agente è **basato sui token**. Ogni chiamata LLM restituisce un conteggio di token, la piattaforma lo converte in centesimi USD usando la tariffa per token del modello, e i centesimi vengono addebitati ai budget dell'agente e del tenant.

### Cosa viene fatturato

- **Tutte le chiamate LLM**, inclusa la chiamata che non produce azioni sugli strumenti ("l'agente ha deciso di non fare nulla"). L'inferenza è a pagamento anche quando non ne risulta alcuna azione.
- **Chiamate in dry-run**. Dry-run significa "non agire, ma chiamare comunque l'LLM" - la chiamata LLM costa lo stesso. Vedi [Modalità Dry-Run](#dry-run-mode).
- **Chiamate di replay**. I replay sono esecuzioni in dry-run contro commenti storici. Costano token. Vedi [Esecuzioni di test (Replays)](#test-runs-replays).

### Cosa non viene fatturato

- **Trigger che non generano mai una chiamata LLM.** I casi dropped-before-LLM (oltre il budget, limitati dalla velocità, incompatibilità di ambito, fatturazione non valida, prevenzione di loop) non costano token. Vedi [Motivi di scarto](#drop-reasons).
- **Dispatch degli strumenti.** Chiamare `pin_comment` o qualsiasi altro strumento non costa di per sé token - solo il round-trip LLM lo fa.
- **`search_memory`.** È sola lettura e non genera un proprio round-trip LLM.

### Costo per esecuzione

Una singola esecuzione dell'agente può chiamare l'LLM più volte - il risultato di ogni chiamata a uno strumento viene reinserito nel modello in modo che possa chiamare un altro strumento o terminare. Quindi `tokensUsed` su una esecuzione è la somma di tutti i round-trip LLM in quella esecuzione.

I maggiori contributori al costo in token per esecuzione:

- **Lunghi [prompt iniziali](#personality-prompt) e [linee guida della community](#community-guidelines)** - vengono inclusi in ogni esecuzione.
- **[Opzioni di contesto](#context-options)** - contesto del thread, cronologia utente, metadati della pagina. Ognuno aggiunge token.
- **Il testo del commento stesso** - commenti lunghi costano di più.
- **Più chiamate a strumenti in una singola esecuzione** - il messaggio di risultato di ogni strumento viene inviato di nuovo al modello.
- **Letture dalla memoria** - `search_memory` restituisce fino a 25 record (capped at 8000 chars total content). La maggior parte di quei byte viene inserita nel prompt successivo.

**Max Tokens Per Trigger** (default 20,000) limita la dimensione della **risposta** per chiamata LLM. Non limita la dimensione dell'input.

### Conversione token in centesimi

La piattaforma applica una singola tariffa per tenant-package (`flexLLMCostCents` per `flexLLMUnit` token). Il costo per token è a livello di pacchetto, non per modello - entrambi i modelli disponibili ([GLM 5.1 e GPT-OSS Turbo](#choosing-a-model)) vengono fatturati allo stesso tasso su un dato pacchetto. La [Vista dettaglio esecuzione](#run-detail-view) mostra il costo per esecuzione nella tua valuta una volta terminata l'esecuzione.

### Dove viene registrato il costo

Ogni esecuzione registra il suo conteggio grezzo di token e il costo per esecuzione. I totali giornalieri e mensili si aggregano nella [Pagina Analytics](#analytics-page).

### Come leggere il costo

- **Costo per esecuzione**: [Vista dettaglio esecuzione](#run-detail-view) -> campo `Cost`.
- **Aggregato giornaliero / mensile**: [Pagina Analytics](#analytics-page) -> grafici di utilizzo del budget e costo giornaliero.
- **Costo per azione**: anche nella Vista dettaglio esecuzione, utile per ottimizzare quando il ciclo di strumenti di un agente è insolitamente lungo.

### Vedi anche

- [Scelta del modello](#choosing-a-model) - la leva maggiore sul costo.
- [Opzioni di contesto](#context-options) - da dove proviene il costo aggiuntivo.
- [Panoramica dei budget](#budgets-overview) - limiti rigidi che prevengono costi fuori controllo.

---