Umfragen funktionieren genauso wie im Web-Widget und werden über dieselben Widget-Einstellungen in Ihrem FastComments-Dashboard aktiviert (oder mit `pollCreation` / `pollVoting` oben).

- Kommentare mit einer Umfrage zeigen diese unter ihrem Text an. Leser stimmen ab, indem sie auf eine Option tippen, können die Ergebnisse vor der Abstimmung ein- oder ausblenden und ihre Stimme ändern. Die Zählungen werden live aktualisiert, wenn andere abstimmen.
- Umfragen können zu einem Datum geschlossen werden, Ergebnisse verbergen, bis Sie abgestimmt haben, und die Wähler für Admins oder für alle sichtbar machen ("Sehen, wer abgestimmt hat").
- Wenn der Betrachter Umfragen erstellen darf, zeigt der Kommentar-Eingabebereich von `FastCommentsView` eine Umfrage-Schaltfläche an, die den Umfrage-Builder öffnet. Die Toolbar-Zeile erscheint dafür, selbst wenn Sie die Toolbar nie aktiviert haben; `sdk.setCommentToolbarEnabled(false)` blendet sie aus, einschließlich der Umfrage-Schaltfläche, und `setPollCreationEnabled(false)` am Eingabefeld entfernt nur die Umfrage-Schaltfläche. Rufen Sie `setCommentToolbarEnabled` auf, bevor Sie die `FastCommentsView` erstellen, da das Eingabefeld dies beim Empfang des SDK ausliest; für eine bereits auf dem Bildschirm sichtbare View rufen Sie stattdessen `getBottomCommentInput().setToolbarVisible(false)` auf. Das Live-Chat-Eingabefeld lässt sie weg, wie der Web-Live-Chat.

Um eine Umfrage aus Ihrer eigenen UI zu veröffentlichen, übergeben Sie ein `CommentPollInput` an `postComment`:

```java
CommentPollInput poll = new CommentPollInput()
        .question("Where should we eat?")
        .options(Arrays.asList("Tacos", "Ramen"))
        // Optional: ein ISO-8601-Schlussdatum, ein PollPrivacy-Wert und Ergebnisse, die bis zur Abstimmung verborgen bleiben.
        .closesAt(Instant.now().plus(1, ChronoUnit.DAYS).toString())
        .privacy(PollPrivacy.NUMBER_2.getValue())
        .requireVoteToSeeResults(true);
sdk.postComment("Lunch vote", null, null, poll, callback);
```

`sdk.votePoll(commentId, optionId, callback)` und `sdk.getPollVoters(commentId, page, search, callback)` decken Abstimmungen und die Wählerliste ab. Umfrage-Farben sind in THEMING.md aufgelistet.