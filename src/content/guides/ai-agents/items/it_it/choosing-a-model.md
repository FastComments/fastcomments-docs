Ogni agente viene eseguito con uno dei due modelli LLM, scelti nella sezione **Modello** del modulo di modifica.

### Le due opzioni

- **GLM 5.1 (DeepInfra) - Smarter, bit slower** - l'impostazione predefinita. Qualità di ragionamento superiore, leggermente più lento per chiamata. Raccomandato per agenti in stile moderazione (`Moderator` template, qualsiasi cosa che chiami `ban_user` o `mark_comment_spam`) dove il costo di una chiamata errata è elevato.

- **GPT-OSS 120B Turbo (DeepInfra) - Faster** - più veloce per chiamata, latenza inferiore. Raccomandato per agenti ad alto volume e basso rischio (agente di benvenuto, che fissa i thread) dove vuoi risposte entro pochi secondi e le conseguenze di una chiamata errata sono minori.

Entrambi i modelli supportano l'invocazione di funzioni, entrambi vengono eseguiti tramite la stessa API compatibile con OpenAI, e condividono gli stessi schemi per strumento - quindi puoi cambiare un agente salvato tra di essi in qualsiasi momento senza altre modifiche di configurazione.

### Differenze di costo

I due modelli hanno costi per token diversi. I [limiti di budget](#budgets-overview) dell'agente sono denominati nella valuta del tuo account, non in token, quindi passare da un modello all'altro cambia quante esecuzioni rientrano nei tuoi limiti giornalieri e mensili. La [Cronologia delle esecuzioni](#run-history) mostra il costo per esecuzione nella tua valuta una volta che un'esecuzione è completata - osservare alcune esecuzioni dopo un cambio è il modo più semplice per valutare il nuovo tasso di consumo.

### Token per esecuzione

L'utilizzo dei token di risposta del modello viene registrato ad ogni trigger come **tokensUsed**. È incluso nei payload dei webhook `trigger.succeeded` e `trigger.failed` (vedi [Payload dei webhook](#webhook-payloads)) e mostrato nella [Vista dettagli esecuzione](#run-detail-view). La quantità dipende da:

- Quanto [Contesto](#context-options) includi - il contesto del thread, la cronologia dell'utente e i metadati della pagina aggiungono token.
- Quanto sono lunghi il tuo [Prompt iniziale](#personality-prompt) e le [Linee guida della community](#community-guidelines).
- Quanti strumenti l'agente invoca in una singola esecuzione (ogni chiamata a uno strumento e il suo risultato fanno il round-trip attraverso il modello).

**Token massimi per trigger** (default 20,000) è il limite superiore per esecuzione, impostato per agente.

### Cambio dei modelli

Puoi cambiare modello nel modulo di modifica in qualsiasi momento. La cronologia delle esecuzioni e le analitiche esistenti mantengono i loro numeri originali di token e costo - vengono registrati al momento dell'esecuzione. Il nuovo modello si applica solo alle esecuzioni che iniziano dopo il salvataggio.

Non esiste un'opzione "use whichever model is cheaper". La scelta è esplicita per ogni agente.