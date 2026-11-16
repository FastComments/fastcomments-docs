Ankete delujejo enako kot v spletnem gradniku in vklopijo se v istih nastavitvah gradnika v vaši nadzorni plošči FastComments (ali z `pollCreation` / `pollVoting` zgoraj).

- Komentarji z anketo prikažejo anketo pod besedilom. Bralci glasujejo s tapkanjem po možnosti, lahko prikažejo ali skrijejo rezultate pred glasovanjem in lahko spremenijo svoj glas. Rezultati se posodabljajo v živo, ko drugi glasujejo.
- Ankete se lahko zaprejo na datum, skrijejo rezultate do vašega glasovanja in naredijo glasovalce vidne skrbnikom ali vsem (»Poglej, kdo je glasoval«).
- Ko si gledalec lahko ustvari ankete, vnosno polje komentarjev v `FastCommentsView` prikaže gumb za anketo, ki odpre graditelje anket. Vrstica z orodno vrstico se za to prikaže, tudi če orodne vrstice nikoli niste vklopili; `sdk.setCommentToolbarEnabled(false)` jo skrije, vključno z gumbom za anketo, `setPollCreationEnabled(false)` pa na vnosnem polju odstrani samo gumb za anketo. Pokličite `setCommentToolbarEnabled` pred ustvarjanjem `FastCommentsView`, saj ga vnosno polje prebere, ko prejme SDK; za pogled, ki je že na zaslonu, pa namesto tega pokličite `getBottomCommentInput().setToolbarVisible(false)`. Vnosno polje klepeta v živo ga izpušča, tako kot spletni klepet v živo.

Za objavo ankete iz lastnega uporabniškega vmesnika podajte `CommentPollInput` funkciji `postComment`:

```java
CommentPollInput poll = new CommentPollInput()
        .question("Where should we eat?")
        .options(Arrays.asList("Tacos", "Ramen"))
        // Neobvezno: ISO 8601 datum zaprtja, vrednost PollPrivacy in skriti rezultati do glasovanja.
        .closesAt(Instant.now().plus(1, ChronoUnit.DAYS).toString())
        .privacy(PollPrivacy.NUMBER_2.getValue())
        .requireVoteToSeeResults(true);
sdk.postComment("Lunch vote", null, null, poll, callback);
```

`sdk.votePoll(commentId, optionId, callback)` in `sdk.getPollVoters(commentId, page, search, callback)` pokrivata glasovanje in seznam glasovalcev. Barve anket so navedene v THEMING.md.