Lo strumento Edit consente all'agente di sostituire il testo di un commento esistente. È distruttivo in un modo in cui la maggior parte degli altri strumenti di moderazione non lo è: sovrascrive contenuti scritti dagli utenti. Riservalo a casi ristretti e ben definiti.

### What it does

L'agente fornisce un ID del commento e un corpo di sostituzione. La piattaforma scrive il nuovo testo nel commento e registra una voce `TextChanged` nel registro di controllo (audit log) del commento, acquisendo sia il testo precedente che quello nuovo. L'originale non viene mai perso - i moderatori possono leggere cosa diceva il commento prima che l'agente lo modificasse.

La sostituzione passa attraverso la stessa pipeline di rendering di una modifica umana: il mascheramento delle volgarità, il parsing delle menzioni, l'estrazione degli hashtag e la gestione di link/immagini si comportano esattamente come se l'autore originale avesse inviato il nuovo testo.

### Scope

Come ogni strumento che muta commenti, Edit è vincolato all'allowlist del trigger - l'agente può modificare solo il commento su cui il trigger è stato attivato, il suo genitore, o un altro commento in ambito dallo stesso contesto del trigger. Un tentativo di prompt-injection per "edit comment XYZ" dove XYZ non è correlato verrà rifiutato lato server prima che l'esecutore venga eseguito.

### Loops

Quando l'agente modifica un commento, la piattaforma attiva un trigger `COMMENT_EDIT` come farebbe per una modifica umana, ma **sopprime l'inoltro ad altri agenti**. Questo impedisce che due agenti che ascoltano entrambi `COMMENT_EDIT` si rispondano a vicenda ping-pongando sulle rispettive modifiche.

### When to allow it

Per agenti che gestiscono la redazione di PII, o per agenti riassuntori/digest che si auto-modificano. La maggior parte degli agenti di moderazione **non** necessita di questo strumento - mark-spam, warn e ban coprono il ciclo di vita tipico.

### Approvals

**Considera fortemente di posizionarlo dietro un'approvazione**, specialmente mentre stai costruendo fiducia nell'agente. Un agente che riscrive le parole di un utente è il tipo di azione che una comunità noterà e a cui reagirà, ed è più difficile da "annullare" dal punto di vista reputazionale rispetto a una cancellazione.

### See also

- [Trigger: Comment Edited](#trigger-comment-edit) - il trigger attivato quando il testo di un commento cambia.
- [Approval Workflow](#approval-workflow) - come mettere lo strumento dietro una revisione umana.