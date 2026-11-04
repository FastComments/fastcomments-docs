**ID del template:** `thread_summarizer`

Il Thread Summarizer pubblica un riassunto neutro di un singolo paragrafo alla fine di un thread lungo. Utilizza un rinvio di 30 minuti in modo che il thread si assesti prima che l'agent lo esamini.

Il prompt integrato istruisce l'agent a non esprimere opinioni personali - questo è fondamentale. Senza di esso, il modello tende a formulazioni del tipo "a mio avviso" che risultano poco adatte sotto il nome visualizzato del tuo account.

### Trigger

- **Nuovo commento pubblicato** (`COMMENT_ADD`).
- **Ritardo del trigger**: 30 minuti (1800 secondi). Vedi [Trigger differiti](#trigger-deferred-delay).

Il ritardo di 30 minuti significa che l'agent viene eseguito una volta, mezz'ora dopo l'arrivo di un commento, su com'è il thread in quel momento. Non si tratta di "riassumere a ogni risposta" - la coda dei trigger differiti raggruppa più eventi di nuovo commento sullo stesso thread, ma non li de-duplica tra finestre separate. Probabilmente vorrai **aggiungere una regola personalizzata nel tuo prompt** come "non pubblicare un nuovo riassunto se l'agent ha già riassunto questo thread nelle ultime 24 ore" e affidarti al contesto più agli [strumenti di memoria](#tools-overview) dell'agent per farla rispettare.

### Strumenti consentiti

- [`write_comment`](#tools-overview) - pubblica il riassunto stesso.
- [`pin_comment`](#tools-overview) - fissa il riassunto in alto così che i lettori lo vedano in cima al thread.
- [`unpin_comment`](#tools-overview) - rimuove il fissaggio di un riassunto precedente dello stesso agent prima di fissare quello nuovo.

Il summarizer non può moderare né interagire con gli utenti.

### Fissaggio del riassunto

L'agent pubblica un nuovo commento con `write_comment`, poi chiama `pin_comment` con l'ID del commento restituito. Nelle esecuzioni successive sullo stesso thread, il prompt lo istruisce a chiamare prima `unpin_comment` sul suo riassunto precedente - la piattaforma stessa **non** applica una regola di un solo commento fissato per thread, quindi lasciare fissato il riassunto precedente comporterà due riassunti fissati affiancati. Spunta "Includi commento principale e risposte precedenti nello stesso thread" in [Opzioni contesto](#context-options) così l'agent può vedere il riassunto precedente fissato.

### Aggiunte consigliate prima di andare in produzione

- **Spunta "Includi commento principale e risposte precedenti nello stesso thread"** in [Opzioni contesto](#context-options). Un summarizer senza contesto del thread è inutile.
- **Regola la soglia minima di dimensione del thread.** "Meno di 5 commenti" è il valore predefinito del prompt, ma nelle community molto attive 10-20 è più appropriato. Modifica direttamente il prompt.
- **Limita a pattern URL specifici** se vuoi riassunti solo sulle pagine di contenuti lunghi, non su annunci o pagine di prodotto. Vedi [Ambito: filtri URL e localizzazione](#scope-url-locale).
- **Monitora i costi.** La sintesi è il template che consuma più token perché legge l'intero thread a ogni esecuzione. Imposta un [budget giornaliero](#budgets-overview) restrittivo prima di passare a Enabled.

### Evitare riassunti ripetuti

L'agent ha accesso a [`save_memory`](#tools-overview) e [`search_memory`](#tools-overview) - puoi estendere il prompt per istruirlo a registrare note "summarized {thread urlId}" e verificarle prima di pubblicare di nuovo. La memoria è condivisa tra tutti gli agent del tuo tenant.