Ankete rade isto kao u veb vidžetu i uključuju se iz istih podešavanja vidžeta u vašem FastComments kontrolnom panelu: `pollCreation` (ko može da kači anketu: onemogućeno, administratori i moderatori, ili svi) i `pollVoting` (prijavljeni korisnici ili bilo ko).

- Komentari sa anketom prikazuju je ispod teksta. Čitaoci glasaju dodirivanjem opcije, mogu da prikažu ili sakriju rezultate pre glasanja i mogu da promene svoj glas. Zbirovi se ažuriraju uživo dok drugi glasaju.
- Ankete mogu da se zatvore na datum, da sakriju rezultate dok ne glasate i da učesnike glasanja učine vidljivim administratorima ili svima ("Pogledaj ko je glasao").
- Kada posmatrač može da kreira ankete, unos komentara u `FastCommentsView` prikazuje dugme za anketu koje otvara kreator ankete. Anketa se objavljuje uz komentar, pa je tekst komentara obavezan. Unos uživo ćaskanja izostavlja to dugme, kao i veb uživo ćaskanje.

Glasanje zahteva prijavljenog korisnika, pa koristite [SSO](#authentication-sso-readme-generated). Server vezuje anonimni glas za kolačić sesije, koji API klijent ne čuva, pa kada je `pollVoting` postavljeno na "bilo ko", posmatrač bez SSO korisnika vidi "Prijavite se da glasate". Ankete koje sakrivaju rezultate dok ne glasate ne dobijaju ažuriranja zbirova uživo; ti glasači vide nove brojeve pri sledećem učitavanju.

Da biste objavili anketu iz sopstvenog UI-ja, prosledite `CommentPollInput` funkciji `postComment`:

```swift
let draft = PollDraft(question: "Where should we eat?", options: ["Tacos", "Ramen"], closesDays: 1)
if draft.validate() == nil {
    try await sdk.postComment(text: "Lunch vote", poll: draft.toInput())
}
```

`sdk.votePoll(commentId:optionId:)` i `sdk.getPollVoters(commentId:page:search:)` pokrivaju glasanje i listu glasača. Boje ankete su navedene pod [Poll Colors](#poll-colors-readme-generated).