**ID del modello:** `thread_summarizer`

Il Thread Summarizer pubblica un sommario neutrale di un solo paragrafo alla fine di un lungo thread. Utilizza un differimento di 30 minuti in modo che il thread si stabilizzi prima che l'agente lo esamini.

### Prompt iniziale integrato

[inline-code-attrs-start title = 'Prompt iniziale del modello Thread Summarizer'; type='text' inline-code-attrs-end]
[inline-code-start]
You post neutral thread summaries. Do not summarize threads that have fewer than 5 comments. For longer threads, summarize the main positions, disagreements, and open questions in one short paragraph. Do not take sides and do not editorialize. After posting the summary, pin it. If a prior summary by you is already pinned on this thread, unpin it before pinning the new one.
[inline-code-end]

L'istruzione "do not editorialize" è fondamentale. Senza di essa il modello tende a usare un formato tipo "in my view" che risulta poco adatto sotto il nome visualizzato del tuo account.

### Attivatori

- **Nuovo commento pubblicato** (`COMMENT_ADD`).
- **Ritardo di attivazione**: 30 minuti (1800 secondi). Vedi [Trigger differiti](#trigger-deferred-delay).

Il ritardo di 30 minuti significa che l'agente viene eseguito una volta, mezz'ora dopo l'arrivo di un commento, rispetto a come appare il thread in quel momento. Non significa "riassumi a ogni risposta" — la coda dei trigger differiti coalesce più eventi di nuovo commento sullo stesso thread, ma non li de-duplica attraverso finestre separate. Probabilmente vorrai **aggiungere una regola personalizzata nel tuo prompt** come "non pubblicare un nuovo sommario se l'agente ha già riassunto questo thread nelle ultime 24 ore" e fare affidamento sul contesto insieme agli [strumenti di memoria](#tools-overview) dell'agente per applicarla.

### Strumenti consentiti

- [`write_comment`](#tools-overview) - pubblica il sommario vero e proprio.
- [`pin_comment`](#tools-overview) - fissa il sommario in modo che i lettori lo vedano in cima al thread.
- [`unpin_comment`](#tools-overview) - rimuove il pin da un sommario precedente dello stesso agente prima di fissare quello nuovo.

Il summarizer non può moderare o interagire con gli utenti.

### Fissare il sommario

L'agente pubblica un nuovo commento con `write_comment`, quindi chiama `pin_comment` con l'ID del commento restituito. Nelle esecuzioni successive sullo stesso thread, il prompt gli indica di chiamare prima `unpin_comment` sul suo precedente sommario — la piattaforma stessa non impone una regola di un solo commento fissato per thread, quindi lasciare il sommario precedente fissato comporterà due sommari fissati affiancati. Seleziona "Include parent comment and prior replies in the same thread" in [Opzioni di contesto](#context-options) affinché l'agente possa vedere il sommario precedentemente fissato.

### Aggiunte consigliate prima di andare in produzione

- **Seleziona "Include parent comment and prior replies in the same thread"** in [Opzioni di contesto](#context-options). Un summarizer senza contesto del thread è inutile.
- **Regola la regola sulla dimensione minima del thread.** "Meno di 5 commenti" è l'impostazione predefinita del prompt, ma nelle community molto attive 10-20 è più appropriato. Modifica direttamente il prompt.
- **Restringi a pattern URL specifici** se vuoi riassunti solo su pagine di formato lungo, non su annunci o pagine prodotto. Vedi [Ambito: filtri URL e locale](#scope-url-locale).
- **Controlla i costi.** La sintetizzazione è il template più dispendioso in termini di token perché legge l'intero thread a ogni esecuzione. Imposta un [budget giornaliero](#budgets-overview) stringente prima di passare a Enabled.

### Evitare riassunti ripetuti

L'agente ha accesso a [`save_memory`](#tools-overview) e [`search_memory`](#tools-overview) - puoi estendere il prompt per istruirlo a registrare note come "summarized {thread urlId}" e verificarle prima di pubblicare di nuovo. La memoria è condivisa tra tutti gli agenti del tuo tenant.