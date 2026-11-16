Ankete rade isto kao u web widgetu, i uključuju se iz istih postavki widgeta u vašem FastComments dashboardu (ili s `pollCreation` / `pollVoting` gore).

- Komentari s anketom prikazuju ispod svog teksta. Čitatelji glasaju dodirom na opciju, mogu prikazati ili sakriti rezultate prije glasovanja i mogu promijeniti svoj glas. Zbrojevi se ažuriraju uživo dok drugi glasaju.
- Ankete se mogu zatvoriti na datum, sakriti rezultate dok ne glasate i učiniti glasače vidljivima administratorima ili svima ("See who voted").
- Kada gledatelj može stvarati ankete, unos komentara u `FastCommentsView` prikazuje gumb za anketu koji otvara graditelj ankete. Redak alatne trake pojavljuje se za to čak i ako nikada niste uključili alatnu traku; `sdk.setCommentToolbarEnabled(false)` je skriva, uključujući gumb za anketu, a `setPollCreationEnabled(false)` na unosu uklanja samo gumb za anketu. Pozovite `setCommentToolbarEnabled` prije stvaranja `FastCommentsView`, budući da unos čita tu postavku kada primi SDK; za prikaz koji je već na ekranu umjesto toga pozovite `getBottomCommentInput().setToolbarVisible(false)`. Unos live chata ga izostavlja, kao i web live chat.

Za objavu ankete iz vlastitog UI-ja, proslijedite `CommentPollInput` u `postComment`:

```java
CommentPollInput poll = new CommentPollInput()
        .question("Where should we eat?")
        .options(Arrays.asList("Tacos", "Ramen"))
        // Neobavezno: ISO 8601 datum zatvaranja, PollPrivacy vrijednost i skriveni rezultati dok ne glasate.
        .closesAt(Instant.now().plus(1, ChronoUnit.DAYS).toString())
        .privacy(PollPrivacy.NUMBER_2.getValue())
        .requireVoteToSeeResults(true);
sdk.postComment("Lunch vote", null, null, poll, callback);
```

`sdk.votePoll(commentId, optionId, callback)` i `sdk.getPollVoters(commentId, page, search, callback)` pokrivaju glasovanje i popis glasača. Boje ankete navedene su u THEMING.md.