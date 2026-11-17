Umfragen funktionieren genauso wie im Web-Widget und werden über dieselben Widget-Einstellungen in Ihrem FastComments-Dashboard aktiviert: `pollCreation` (wer eine Umfrage anhängen darf: deaktiviert, Admins und Moderatoren oder alle) und `pollVoting` (eingeloggte Benutzer oder jeder).

- Kommentare mit einer Umfrage zeigen diese unter ihrem Text an. Leser stimmen ab, indem sie auf eine Option tippen, können die Ergebnisse vor der Abstimmung ein- oder ausblenden und ihre Stimme ändern. Die Zählungen werden live aktualisiert, wenn andere abstimmen.
- Umfragen können an einem Datum enden, die Ergebnisse verbergen, bis Sie abgestimmt haben, und die Wähler für Admins oder für alle sichtbar machen („Sehen, wer abgestimmt hat").
- Wenn der Betrachter Umfragen erstellen darf, zeigt der Kommentar-Eingabebereich von `FastComments` eine Umfrage-Schaltfläche, die den Umfrage-Builder öffnet. Eine Umfrage wird zusammen mit einem Kommentar gepostet, daher ist der Kommentartext erforderlich. Die Live-Chat-Eingabe lässt die Schaltfläche weg, genau wie der Web-Live-Chat.

Die Abstimmung erfordert einen eingeloggten Benutzer, verwenden Sie daher [SSO](#authentication-sso-readme-generated). Der Server verknüpft eine anonyme Abstimmung mit einem Session-Cookie, den der API-Client nicht speichert. Wenn `pollVoting` auf „jeder" gesetzt ist, sieht ein Betrachter ohne SSO-Benutzer daher „Log in to vote". Umfragen, die die Ergebnisse bis zur Abstimmung verbergen, erhalten keine Live-Zählungs-Updates; diese Wähler sehen neue Zahlen bei ihrem nächsten Laden.

Um eine Umfrage über Ihre eigene Benutzeroberfläche zu posten, übergeben Sie ein `CommentPollInput` an `postComment`:

```swift
let draft = PollDraft(question: "Where should we eat?", options: ["Tacos", "Ramen"], closesDays: 1)
if draft.validate() == nil {
    try await sdk.postComment(text: "Lunch vote", poll: draft.toInput())
}
```

`sdk.votePoll(commentId:optionId:)` und `sdk.getPollVoters(commentId:page:search:)` decken Abstimmung und die Wählerliste ab. Umfrage-Farben sind unter [Poll Colors](#poll-colors-readme-generated) aufgelistet.