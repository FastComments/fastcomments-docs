Ankete rade isto kao u web widgetu i uključuju se iz istih postavki widgeta u vašoj FastComments upravljačkoj ploči: `pollCreation` (tko može priložiti anketu: onemogućeno, administratori i moderatori ili svi) i `pollVoting` (prijavljeni korisnici ili itko).

- Komentari s anketom prikazuju je ispod svog teksta. Čitatelji glasaju dodirom na opciju, mogu prikazati ili sakriti rezultate prije glasovanja i mogu promijeniti svoj glas. Zbrojevi se ažuriraju uživo dok drugi glasaju.
- Ankete se mogu zatvoriti na datum, sakriti rezultate dok ne glasate i učiniti glasače vidljivima administratorima ili svima ("Prikaži tko je glasao").
- Kada gledatelj može stvarati ankete, unos komentara u `FastCommentsView` prikazuje gumb za anketu koji otvara kreator ankete. Anketa se objavljuje s komentarom, pa je tekst komentara obavezan. Unos za live chat ne prikazuje taj gumb, kao ni web live chat.

Glasovanje zahtijeva prijavljenog korisnika, pa koristite [SSO](#authentication-sso-readme-generated). Server povezuje anonimni glas s kolačićem sesije, koji API klijent ne čuva, pa kada je `pollVoting` postavljen na itko, gledatelj bez SSO korisnika vidi "Prijavite se za glasovanje". Ankete koje skrivaju rezultate dok ne glasate ne dobivaju ažuriranja zbrojeva uživo; ti glasači vide nove brojke pri sljedećem učitavanju.

Za objavu ankete iz vlastitog UI-ja, proslijedite `CommentPollInput` funkciji `postComment`:

```swift
let draft = PollDraft(question: "Where should we eat?", options: ["Tacos", "Ramen"], closesDays: 1)
if draft.validate() == nil {
    try await sdk.postComment(text: "Lunch vote", poll: draft.toInput())
}
```

`sdk.votePoll(commentId:optionId:)` i `sdk.getPollVoters(commentId:page:search:)` pokrivaju glasovanje i popis glasača. Boje anketa navedene su pod [Poll Colors](#poll-colors-readme-generated).