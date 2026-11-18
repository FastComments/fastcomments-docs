Afstemninger fungerer på samme måde som i web-widget'en, og aktiveres fra de samme widget-indstillinger i dit FastComments-dashboard: `pollCreation` (hvem kan vedhæfte en afstemning: deaktiveret, admins og moderatorer, eller alle) og `pollVoting` (loggede brugere eller alle).

- Kommentarer med en afstemning viser den under deres tekst. Læsere stemmer ved at trykke på en mulighed, kan vise eller skjule resultater før afstemning og kan ændre deres stemme. Optællinger opdateres live, efterhånden som andre stemmer.
- Afstemninger kan lukke på en dato, skjule resultater, indtil man stemmer, og gøre stemmere synlige for admins eller for alle ("Se, hvem der stemte").
- Når seeren må oprette afstemninger, viser `FastCommentsView`'s kommentarinput en afstemningsknap, der åbner afstemningsbyggeren. En afstemning postes med en kommentar, så kommentarteksten er påkrævet. Live chat-inputtet udelader knappen, ligesom web live chat.

Afstemning kræver en logget ind-bruger, så brug [SSO](#authentication-sso-readme-generated). Serveren knytter en anonym stemme til en session-cookie, som API-klienten ikke gemmer, så med `pollVoting` sat til alle ser en seer uden SSO-bruger "Log ind for at stemme". Afstemninger, der skjuler resultater, indtil man stemmer, får ikke live opdateringer af optællingen; de stemmere ser nye tal ved deres næste indlæsning.

For at poste en afstemning fra din egen UI, send en `CommentPollInput` til `postComment`:

```swift
let draft = PollDraft(question: "Where should we eat?", options: ["Tacos", "Ramen"], closesDays: 1)
if draft.validate() == nil {
    try await sdk.postComment(text: "Lunch vote", poll: draft.toInput())
}
```

`sdk.votePoll(commentId:optionId:)` og `sdk.getPollVoters(commentId:page:search:)` dækker afstemning og listen over stemmere. Afstemningsfarver er listet under [Poll Colors](#theming-readme-generated).