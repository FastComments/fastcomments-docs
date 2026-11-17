Polls werken hetzelfde als in de web widget en worden aangezet via dezelfde widgetinstellingen in je FastComments dashboard: `pollCreation` (wie een poll mag toevoegen: uitgeschakeld, admins en moderators, of iedereen) en `pollVoting` (ingelogde gebruikers of iedereen).

- Reacties met een poll tonen deze onder hun tekst. Lezers stemmen door op een optie te tikken, kunnen de resultaten vóór het stemmen tonen of verbergen, en kunnen hun stem wijzigen. De tussenstanden worden live bijgewerkt terwijl anderen stemmen.
- Polls kunnen op een datum aflopen, resultaten verbergen tot je stemt, en stemmers zichtbaar maken voor admins of voor iedereen ("See who voted").
- Wanneer de kijker polls mag aanmaken, toont de reactie-invoer van `FastCommentsView` een poll-knop die de poll-builder opent. Een poll wordt geplaatst met een reactie, dus de reactietekst is verplicht. De live chat-invoer laat de knop weg, net als de web live chat.

Stemmen vereist een ingelogde gebruiker, dus gebruik [SSO](#authentication-sso-readme-generated). De server koppelt een anonieme stem aan een sessiecookie, die de API-client niet bewaart, dus met `pollVoting` ingesteld op iedereen ziet een kijker zonder SSO-gebruiker "Log in to vote". Polls die resultaten verbergen tot je stemt krijgen geen live tussenstand-updates; die stemmers zien nieuwe cijfers bij hun volgende load.

Om een poll te plaatsen vanuit je eigen UI, geef een `CommentPollInput` door aan `postComment`:

```swift
let draft = PollDraft(question: "Where should we eat?", options: ["Tacos", "Ramen"], closesDays: 1)
if draft.validate() == nil {
    try await sdk.postComment(text: "Lunch vote", poll: draft.toInput())
}
```

`sdk.votePoll(commentId:optionId:)` en `sdk.getPollVoters(commentId:page:search:)` dekken stemmen en de stemmerslijst af. Poll-kleuren staan vermeld onder [Poll Colors](#poll-colors-readme-generated).