**Template ID:** `top_comment_pinner`

Il Top Comment Pinner sorveglia i commenti di primo livello che superano una soglia di voti e li fissa — sostituendo ciò che era stato precedentemente fissato nello stesso thread.

Il prompt integrato istruisce l'agente a ignorare le risposte (la pin funziona sui thread, quindi fissare una risposta è raramente utile) e a filtrare i contenuti promozionali (così l'agente non valorizza lo spam di link popolare).

### Attivatori

- **Un commento supera una soglia di voti** (`COMMENT_VOTE_THRESHOLD`, soglia di voto predefinita: 10).

Il trigger si attiva quando i voti netti del commento (`up - down`) raggiungono la soglia configurata. Regola il numero nel modulo di modifica in base a quanto sono attivi i tuoi thread - 10 è un valore sensato per siti moderatamente attivi.

### Strumenti consentiti

- [`pin_comment`](#tools-overview)
- [`unpin_comment`](#tools-overview)

Il pinning non è distruttivo - può essere annullato immediatamente - quindi questo template di solito viene eseguito senza approvazioni.

### Aggiunte consigliate prima della pubblicazione

- **Seleziona "Includi il commento padre e le risposte precedenti nello stesso thread"** in [Opzioni di contesto](#context-options). Senza il contesto del thread l'agente non può determinare in modo affidabile se c'è già un commento fissato da rimuovere.
- **Regola la soglia di voti** in base al tuo sito. Nei thread molto attivi 10 avviene troppo spesso; nei thread tranquilli 10 potrebbe non verificarsi mai.
- **Considera di limitare per URL** se vuoi i commenti fissati solo in alcune sezioni del sito — per esempio thread delle notizie, ma non quelli degli annunci.

### Nota sul pinning duplicato

Il prompt dell'agente gli ordina di rimuovere il pin prima di fissarne uno nuovo, ma se il modello salta questo passaggio la piattaforma stessa non applica la regola di un solo pin per thread (puoi averne più di uno). Se il pinning duplicato è un problema sul tuo sito, sottoponi `pin_comment` ad approvazione e rivedi ciascuno — oppure scrivi un prompt più restrittivo.