I sondaggi funzionano come nel widget web e si attivano dalle stesse impostazioni del widget nella tua dashboard FastComments (oppure con `pollCreation` / `pollVoting` sopra).

- I commenti con un sondaggio lo mostrano sotto il loro testo. I lettori votano toccando un'opzione, possono mostrare o nascondere i risultati prima di votare e possono cambiare il loro voto. I conteggi si aggiornano in tempo reale mentre altri votano.
- I sondaggi possono chiudersi a una data, nascondere i risultati finché non voti e rendere gli elettori visibili agli amministratori o a tutti ("Vedi chi ha votato").
- Quando l'utente può creare sondaggi, l'input dei commenti di `FastCommentsView` mostra un pulsante per i sondaggi che apre il generatore di sondaggi. La riga della toolbar appare per questo anche se non hai mai attivato la toolbar; `sdk.setCommentToolbarEnabled(false)` la nasconde, pulsante dei sondaggi incluso, e `setPollCreationEnabled(false)` sull'input rimuove solo il pulsante dei sondaggi. Chiama `setCommentToolbarEnabled` prima di creare il `FastCommentsView`, poiché l'input lo legge quando riceve l'SDK; per una vista già sullo schermo, chiama invece `getBottomCommentInput().setToolbarVisible(false)`. L'input della live chat lo omette, come la live chat web.

Per pubblicare un sondaggio dalla tua UI, passa un `CommentPollInput` a `postComment`:

```java
CommentPollInput poll = new CommentPollInput()
        .question("Where should we eat?")
        .options(Arrays.asList("Tacos", "Ramen"))
        // Opzionale: una data di chiusura ISO 8601, un valore PollPrivacy e risultati nascosti finché non voti.
        .closesAt(Instant.now().plus(1, ChronoUnit.DAYS).toString())
        .privacy(PollPrivacy.NUMBER_2.getValue())
        .requireVoteToSeeResults(true);
sdk.postComment("Lunch vote", null, null, poll, callback);
```

`sdk.votePoll(commentId, optionId, callback)` e `sdk.getPollVoters(commentId, page, search, callback)` coprono la votazione e l'elenco degli elettori. I colori dei sondaggi sono elencati in THEMING.md.