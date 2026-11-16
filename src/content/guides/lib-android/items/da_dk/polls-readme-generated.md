Afstemninger fungerer på samme måde som i web-widget'en, og aktiveres fra de samme widget-indstillinger i dit FastComments-dashboard (eller med `pollCreation` / `pollVoting` ovenfor).

- Kommentarer med en afstemning viser den under deres tekst. Læsere stemmer ved at trykke på en mulighed, kan vise eller skjule resultater før afstemning og kan ændre deres stemme. Optællinger opdateres live, efterhånden som andre stemmer.
- Afstemninger kan lukke på en dato, skjule resultater, indtil du stemmer, og gøre stemmere synlige for admins eller for alle ("Se, hvem der stemte").
- Når seeren kan oprette afstemninger, viser `FastCommentsView`'s kommentarinput en afstemningsknap, der åbner afstemningsbyggeren. Værktøjslinjerækken vises til det, selv hvis du aldrig aktiverede værktøjslinjen; `sdk.setCommentToolbarEnabled(false)` skjuler den, inklusive afstemningsknappen, og `setPollCreationEnabled(false)` på inputtet fjerner kun afstemningsknappen. Kald `setCommentToolbarEnabled`, før du opretter `FastCommentsView`, da inputtet læser den, når det modtager SDK'en; for en visning, der allerede er på skærmen, kald i stedet `getBottomCommentInput().setToolbarVisible(false)`. Live chat-inputtet udelader den, ligesom web live chat.

For at sende en afstemning fra din egen UI, send en `CommentPollInput` til `postComment`:

```java
CommentPollInput poll = new CommentPollInput()
        .question("Where should we eat?")
        .options(Arrays.asList("Tacos", "Ramen"))
        // Valgfrit: en ISO 8601-lukkedato, en PollPrivacy-værdi og resultater skjult, indtil du stemmer.
        .closesAt(Instant.now().plus(1, ChronoUnit.DAYS).toString())
        .privacy(PollPrivacy.NUMBER_2.getValue())
        .requireVoteToSeeResults(true);
sdk.postComment("Lunch vote", null, null, poll, callback);
```

`sdk.votePoll(commentId, optionId, callback)` og `sdk.getPollVoters(commentId, page, search, callback)` dækker afstemning og listen over stemmere. Afstemningsfarver er listet i THEMING.md.