Lo strumento Edit consente all'agente di sostituire il testo di un commento esistente. È distruttivo in un modo in cui la maggior parte degli altri strumenti di moderazione non lo è: sovrascrive contenuti creati dagli utenti. Riservatelo a casi ristretti e chiaramente definiti.

### Cosa fa

L'agente passa un ID commento e un nuovo corpo. La piattaforma scrive il nuovo testo nel commento e registra una voce `TextChanged` nel log di audit del commento, acquisendo sia il testo precedente che quello nuovo. L'originale non viene mai perso - i moderatori possono leggere cosa diceva il commento prima che l'agente lo modificasse.

La sostituzione passa attraverso la stessa pipeline di rendering di una modifica umana: mascheramento delle parolacce, parsing delle menzioni, estrazione degli hashtag e gestione di link/immagini si comportano esattamente come se l'autore originale avesse inviato il nuovo testo.

### Ambito

Come ogni strumento che modifica i commenti, Edit è vincolato alla allowlist del trigger - l'agente può modificare solo il commento su cui il trigger è scattato, il suo genitore, o un altro commento nell'ambito dello stesso contesto di trigger. Un tentativo di prompt injection per "modificare il commento XYZ" dove XYZ non è correlato verrà rifiutato lato server prima che l'executor venga eseguito.

### Loop

Quando l'agente modifica un commento, la piattaforma attiva un trigger `COMMENT_EDIT` come farebbe per una modifica umana, ma **sopprime l'invio agli altri agenti**. Questo impedisce a due agenti che entrambi ascoltano `COMMENT_EDIT` di fare ping-pong sulle modifiche reciproche.

### Quando consentirlo

Per agenti che gestiscono la redazione di PII, o per agenti di riepilogo/digest che si auto-modificano. La maggior parte degli agenti di moderazione **non** ha bisogno di questo strumento - mark-spam, warn e ban coprono il ciclo di vita tipico.

### Approvazioni

**Valutate fortemente l'inserimento di un gate di approvazione**, soprattutto mentre state costruendo fiducia nell'agente. Un agente che riscrive le parole di un utente è il tipo di azione che una community noterà e a cui reagirà, ed è più difficile da "annullare" in termini di reputazione rispetto a un'eliminazione.

### Vedi anche

- [Trigger: Comment Edited](#trigger-comment-edit) - il trigger attivato quando il testo di un commento cambia.
- [Approval Workflow](#approval-workflow) - come sottoporre lo strumento a revisione umana.