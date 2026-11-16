Ankete rade isto kao u veb vidžetu i uključuju se iz istih podešavanja vidžeta u vašem FastComments kontrolnom panelu (ili putem `pollCreation` / `pollVoting` iznad).

- Komentari sa anketom prikazuju je ispod teksta. Čitaoci glasaju dodirivanjem opcije, mogu da prikažu ili sakriju rezultate pre glasanja i mogu da promene svoj glas. Zbirovi se ažuriraju uživo dok drugi glasaju.
- Ankete mogu da se zatvore na određeni datum, da sakriju rezultate dok ne glasate i da učesnike učine vidljivim administratorima ili svima ("Pogledaj ko je glasao").
- Kada posmatrač može da kreira ankete, polje za unos komentara u `FastCommentsView` prikazuje dugme za anketu koje otvara kreator ankete. Red sa alatkama se pojavljuje za to čak i ako nikada niste uključili alatnu traku; `sdk.setCommentToolbarEnabled(false)` je sakriva, uključujući i dugme za anketu, a `setPollCreationEnabled(false)` na polju za unos uklanja samo dugme za anketu. Pozovite `setCommentToolbarEnabled` pre kreiranja `FastCommentsView`, pošto polje za unos to pročita kada primi SDK; za prikaz koji je već na ekranu, umesto toga pozovite `getBottomCommentInput().setToolbarVisible(false)`. Polje za unos u live chat-u ga izostavlja, kao i veb live chat.

Da biste objavili anketu iz sopstvenog UI-ja, prosledite `CommentPollInput` funkciji `postComment`:

```java
CommentPollInput poll = new CommentPollInput()
        .question("Where should we eat?")
        .options(Arrays.asList("Tacos", "Ramen"))
        // Opciono: ISO 8601 datum zatvaranja, PollPrivacy vrednost i skriveni rezultati dok ne glasate.
        .closesAt(Instant.now().plus(1, ChronoUnit.DAYS).toString())
        .privacy(PollPrivacy.NUMBER_2.getValue())
        .requireVoteToSeeResults(true);
sdk.postComment("Lunch vote", null, null, poll, callback);
```

`sdk.votePoll(commentId, optionId, callback)` i `sdk.getPollVoters(commentId, page, search, callback)` pokrivaju glasanje i listu glasača. Boje ankete su navedene u THEMING.md.