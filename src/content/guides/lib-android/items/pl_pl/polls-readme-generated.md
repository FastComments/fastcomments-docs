Ankiety działają tak samo jak w widżecie webowym i włączają się z tych samych ustawień widżetu w panelu FastComments (lub za pomocą `pollCreation` / `pollVoting` powyżej).

- Komentarze z ankietą wyświetlają ją pod treścią komentarza. Czytelnicy głosują, dotykając opcji, mogą pokazać lub ukryć wyniki przed głosowaniem oraz mogą zmienić swój głos. Wyniki aktualizują się na żywo, gdy inni głosują.
- Ankiety mogą zamykać się w określonym dniu, ukrywać wyniki do momentu oddania głosu oraz udostępniać listę głosujących administratorom lub wszystkim („See who voted").
- Gdy widz może tworzyć ankiety, pole wprowadzania komentarza w `FastCommentsView` pokazuje przycisk ankiety otwierający kreator ankiet. Wiersz paska narzędzi pojawia się dla niego nawet, jeśli nigdy nie włączyłeś paska narzędzi; `sdk.setCommentToolbarEnabled(false)` go ukrywa, łącznie z przyciskiem ankiety, a `setPollCreationEnabled(false)` na polu wprowadzania usuwa tylko przycisk ankiety. Wywołaj `setCommentToolbarEnabled` przed utworzeniem `FastCommentsView`, ponieważ pole wprowadzania odczytuje tę wartość przy otrzymaniu SDK; dla widoku już wyświetlanego na ekranie wywołaj zamiast tego `getBottomCommentInput().setToolbarVisible(false)`. Pole wprowadzania czatu na żywo go pomija, podobnie jak czat na żywo w wersji webowej.

Aby opublikować ankietę z własnego interfejsu, przekaż `CommentPollInput` do `postComment`:

```java
CommentPollInput poll = new CommentPollInput()
        .question("Where should we eat?")
        .options(Arrays.asList("Tacos", "Ramen"))
        // Opcjonalnie: data zamknięcia w formacie ISO 8601, wartość PollPrivacy oraz ukryte wyniki do momentu oddania głosu.
        .closesAt(Instant.now().plus(1, ChronoUnit.DAYS).toString())
        .privacy(PollPrivacy.NUMBER_2.getValue())
        .requireVoteToSeeResults(true);
sdk.postComment("Lunch vote", null, null, poll, callback);
```

`sdk.votePoll(commentId, optionId, callback)` oraz `sdk.getPollVoters(commentId, page, search, callback)` obsługują głosowanie i listę głosujących. Kolory ankiet są opisane w pliku THEMING.md.