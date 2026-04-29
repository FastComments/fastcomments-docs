Attiva l'agente ogni volta che viene pubblicato un nuovo commento su una pagina coperta dall'[ambito](#scope-url-locale) dell'agente.

### Contesto che l'agente riceve

- Il nuovo commento completo - testo, autore, voti, ID del genitore, ID dell'URL della pagina.
- Facoltativo: il commento genitore e le risposte precedenti nello stesso thread, se il [contesto del thread](#context-options) è attivato.
- Facoltativo: fattore di fiducia del commentatore, età dell'account, storico dei ban e commenti recenti, se il [contesto della cronologia utente](#context-options) è attivato.
- Facoltativo: metadata della pagina, se il [contesto della pagina](#context-options) è attivato.

### Da notare

- Il trigger si attiva **dopo** che il commento è stato salvato. L'agente può farvi riferimento direttamente nelle chiamate agli strumenti.
- Non si attiva per i commenti scritti da un altro agente nello stesso tenant.
- Si attiva sia per commenti verificati che non verificati. Se il tuo tenant richiede l'approvazione di un moderatore prima che un commento sia visibile (vedi [Come funzionano le approvazioni](/guide-moderation.html#moderation-approvals) nella guida sulla moderazione), il trigger si attiva quando il commento viene creato, non quando viene approvato successivamente. Il bot moderatore può essere istruito ad approvare i commenti per te dopo la revisione.

### Usi comuni

- **Moderazione** - verificare il commento rispetto alle linee guida della community, contrassegnare come spam o avvertire i nuovi utenti.
- **Messaggio di benvenuto** - anche se [Trigger: Primo commento di un nuovo utente](#trigger-new-user-first-comment) di solito è più adatto per i saluti poiché si attiva una sola volta per utente.
- **Riepilogo del thread** - solitamente abbinato a un [ritardo del trigger](#trigger-deferred-delay) in modo che il thread si stabilizzi prima che l'agente venga eseguito.

---