Si attiva quando il numero di segnalazioni di un commento raggiunge **esattamente** la soglia configurata.

### Configurazione richiesta

- **Soglia di segnalazioni** - intero >= 1. Il trigger si attiva nel momento in cui `flagCount === flagThreshold`. Non si attiva di nuovo su segnalazioni successive oltre la soglia.

Se la soglia è 3 e tre utenti segnalano il commento, l'agente si attiva una sola volta alla terza segnalazione. Una quarta, quinta o sesta segnalazione **non** lo riattiva.

### Contesto che riceve l'agente

- Il commento segnalato.
- Storico opzionale del thread / dell'utente / della pagina come configurato.
- Il conteggio delle segnalazioni è nel blocco del commento come `Flag Count: N`.

### Note

- Il trigger si attiva solo quando il commento supera la soglia dal basso tramite il percorso di gestione delle segnalazioni della piattaforma (dove `didIncrement === true`). Scritture dirette sul DB che impostano `flagCount` al valore della soglia non lo attivano; segnalazioni oltre la soglia non lo riattivano neppure.
- Non include chi ha segnalato il commento - le segnalazioni sono anonime per l'agente. Se vuoi analizzare gli utenti che segnalano, recuperali dai tuoi dati.
- Un ritardo del trigger (vedi [Deferred Triggers](#trigger-deferred-delay)) è *fortemente* raccomandato per questo trigger - le segnalazioni spesso arrivano a raffica durante un thread acceso, e un piccolo ritardo lascia che la situazione si stabilizzi prima che l'agente agisca.

### Usi comuni

- **Revisione moderazione** - un commento segnalato è il segnale canonico "gli umani pensano che questo potrebbe essere problematico". Il [Moderator template](#template-moderator) si iscrive a questo trigger per impostazione predefinita con una soglia di segnalazioni pari a 3.
- **Arricchimento della coda di pre-moderazione** - l'agente esegue un primo controllo e o marca il commento per la revisione (con `mark_comment_reviewed`) o lo incrementa ulteriormente.
- **Anti-brigading** - combina questo trigger con il [contesto della cronologia utente](#context-options) e lascia che l'agente veda precedenti ban/segnali di contenuto duplicato prima di agire.

### Raccomandazioni per l'uso combinato

Iscriviti a **entrambi** `COMMENT_ADD` e `COMMENT_FLAG_THRESHOLD` se vuoi un agente di moderazione che catturi i casi ovvi al primo colpo e rivaluti quelli borderline una volta che le segnalazioni si accumulano. I due eventi si attivano indipendentemente - l'agente verrà eseguito due volte se entrambi sono sottoscritti e si attivano entrambi, ma la seconda esecuzione vede lo stato ora segnalato.