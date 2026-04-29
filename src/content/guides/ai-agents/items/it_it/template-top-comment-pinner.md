**ID del template:** `top_comment_pinner`

Il Top Comment Pinner monitora i commenti di primo livello che superano una soglia di voti e li fissa, sostituendo qualsiasi commento precedentemente fissato nello stesso thread.

### Prompt iniziale incorporato

[inline-code-attrs-start title = 'Prompt iniziale del template Top Comment Pinner'; type='text' inline-code-attrs-end]
[inline-code-start]
Fissi i migliori commenti di primo livello in un thread. Quando un commento raggiunge la soglia di voti, fissalo se il contenuto è sostanziale e non promozionale. Rimuovi prima qualsiasi commento precedentemente fissato nello stesso thread. Non fissare le risposte, solo i commenti di primo livello.
[inline-code-end]

L'istruzione «non fissare le risposte» è importante: il fissaggio funziona per thread, quindi fissare una risposta è raramente utile. Il filtro «non promozionale» impedisce all'agente di amplificare un commento di spam con link, anche se è popolare.

### Eventi di attivazione

- **Un commento supera una soglia di voti** (`COMMENT_VOTE_THRESHOLD`, soglia di voti predefinita: 10).

Il trigger si attiva quando i voti netti del commento (`up - down`) raggiungono la soglia configurata. Regola questo numero nel modulo di modifica in base all'attività dei tuoi thread - 10 è un valore sensato come predefinito per siti moderatamente attivi.

### Strumenti consentiti

- [`pin_comment`](#tools-overview)
- [`unpin_comment`](#tools-overview)

Il fissaggio non è distruttivo - può essere annullato istantaneamente - quindi questo template di solito viene eseguito senza approvazioni.

### Raccomandazioni consigliate prima di andare in produzione

- **Seleziona "Include parent comment and prior replies in the same thread"** in [Context Options](#context-options). Senza il contesto del thread l'agente non può stabilire in modo affidabile se esiste già un commento fissato da rimuovere.
- **Regola la soglia di voti** per il tuo sito. Nei thread molto attivi 10 accade troppo spesso; nei thread poco attivi 10 potrebbe non verificarsi mai.
- **Valuta di limitare per URL** se vuoi commenti fissati solo in alcune sezioni del tuo sito - ad esempio i thread delle notizie ma non quelli degli annunci.

### Nota sul fissaggio duplicato

Il prompt dell'agente gli dà istruzioni di rimuovere il pin prima di fissare, ma se il modello salta questo passaggio la piattaforma non applica una regola di un solo commento fissato per thread (puoi averne più di uno). Se i pin duplicati sono un problema sul tuo sito, proteggi `pin_comment` richiedendo approvazione e revisiona ciascuno - oppure scrivi un prompt più restrittivo.

---