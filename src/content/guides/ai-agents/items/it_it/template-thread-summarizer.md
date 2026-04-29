**Template ID:** `thread_summarizer`

Il Thread Summarizer pubblica un riassunto neutro, in un unico paragrafo, alla fine di un thread lungo. Usa un rinvio di 30 minuti in modo che il thread si assesti prima che l'agente lo esamini.

Il prompt incorporato istruisce l'agente a non editorializzare - questo è fondamentale. Senza questa indicazione il modello tende a usare formulazioni del tipo "a mio avviso" che risultano poco appropriate sotto il nome visualizzato del vostro account.

### Triggers

- **New comment posted** (`COMMENT_ADD`).
- **Trigger delay**: 30 minutes (1800 seconds). See [Deferred Triggers](#trigger-deferred-delay).

Il ritardo di 30 minuti significa che l'agente viene eseguito una sola volta, mezz'ora dopo l'arrivo di un commento, rispetto a come appare il thread in quel momento. Non significa "riassumi ad ogni risposta" - la coda dei trigger differiti aggrega più eventi di nuovo commento sullo stesso thread, ma non li de-duplica attraverso finestre separate. Probabilmente vorrete **aggiungere una regola personalizzata nel vostro prompt** come "do not post a new summary if the agent has already summarized this thread in the last 24 hours" e fare affidamento sul contesto più gli [memory tools](#tools-overview) dell'agente per applicarla.

### Allowed tools

- [`write_comment`](#tools-overview) - pubblica il riassunto stesso.
- [`pin_comment`](#tools-overview) - fissa il riassunto in modo che i lettori lo vedano in cima al thread.
- [`unpin_comment`](#tools-overview) - rimuove il fissaggio di un riassunto precedente dello stesso agente prima di fissare quello nuovo.

Il riassuntore non può moderare né interagire con gli utenti.

### Pinning the summary

L'agente pubblica un nuovo commento con `write_comment`, poi chiama `pin_comment` con l'ID del commento restituito. Nelle esecuzioni successive sullo stesso thread, il prompt gli indica di chiamare prima `unpin_comment` sul suo riassunto precedente - la piattaforma stessa **non** applica una regola di un solo commento fissato per thread, quindi lasciare il riassunto precedente fissato comporterà la presenza di due riassunti fissati affiancati. Seleziona "Include parent comment and prior replies in the same thread" in [Context Options](#context-options) in modo che l'agente possa vedere il riassunto fissato precedente.

### Recommended additions before going live

- **Seleziona "Include parent comment and prior replies in the same thread"** in [Context Options](#context-options). Un riassuntore senza contesto del thread è inutile.
- **Regola la soglia minima di dimensione del thread.** "Meno di 5 commenti" è l'impostazione predefinita del prompt, ma nelle community molto attive 10-20 è più appropriato. Modifica direttamente il prompt.
- **Restringi a pattern URL specifici** se vuoi riassunti solo su pagine di contenuto esteso, non su annunci o pagine prodotto. Vedi [Scope: URL and Locale Filters](#scope-url-locale).
- **Occhio ai costi.** La sintesi è il template che consuma più token perché legge l'intero thread ad ogni esecuzione. Imposta un [budget giornaliero](#budgets-overview) stringente prima di attivarlo.

### Avoiding repeat summaries

L'agente ha accesso a [`save_memory`](#tools-overview) e [`search_memory`](#tools-overview) - puoi estendere il prompt per istruirlo a registrare note "summarized {thread urlId}" e controllarle prima di pubblicare nuovamente. La memoria è condivisa tra tutti gli agenti nel tuo tenant.