Polls werken hetzelfde als in de web widget en worden ingeschakeld via dezelfde widgetinstellingen in je FastComments dashboard (of met `pollCreation` / `pollVoting` hierboven).

- Opmerkingen met een poll tonen deze onder hun tekst. Lezers stemmen door op een optie te tikken, kunnen resultaten tonen of verbergen vóór het stemmen, en kunnen hun stem wijzigen. De tussenstanden worden live bijgewerkt terwijl anderen stemmen.
- Polls kunnen op een datum sluiten, resultaten verbergen totdat je stemt, en stemmers zichtbaar maken voor admins of voor iedereen ("See who voted").
- Wanneer de kijker polls mag aanmaken, toont de commentarinvoer van `FastCommentsView` een poll-knop die de poll-builder opent. De werkbalkrij verschijnt hiervoor zelfs als je de werkbalk nooit hebt ingeschakeld; `sdk.setCommentToolbarEnabled(false)` verbergt deze, inclusief de poll-knop, en `setPollCreationEnabled(false)` op de invoer verwijdert alleen de poll-knop. Roep `setCommentToolbarEnabled` aan vóór het aanmaken van de `FastCommentsView`, omdat de invoer dit uitleest wanneer deze de SDK ontvangt; voor een weergave die al op het scherm staat, roep je in plaats daarvan `getBottomCommentInput().setToolbarVisible(false)` aan. De live chat-invoer laat dit weg, net als de web live chat.

Om een poll te plaatsen vanuit je eigen UI, geef je een `CommentPollInput` door aan `postComment`:

```java
CommentPollInput poll = new CommentPollInput()
        .question("Where should we eat?")
        .options(Arrays.asList("Tacos", "Ramen"))
        // Optioneel: een ISO 8601 sluitingsdatum, een PollPrivacy waarde, en resultaten verborgen totdat je stemt.
        .closesAt(Instant.now().plus(1, ChronoUnit.DAYS).toString())
        .privacy(PollPrivacy.NUMBER_2.getValue())
        .requireVoteToSeeResults(true);
sdk.postComment("Lunch vote", null, null, poll, callback);
```

`sdk.votePoll(commentId, optionId, callback)` en `sdk.getPollVoters(commentId, page, search, callback)` dekken het stemmen en de stemmerslijst af. Poll-kleuren staan vermeld in THEMING.md.