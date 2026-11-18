Ankete delujejo enako kot v spletnem gradniku in vklopijo se iz istih nastavitev gradnika v vaši nadzorni plošči FastComments: `pollCreation` (kdo lahko priloži anketo: onemogočeno, skrbniki in moderatorji ali vsi) in `pollVoting` (prijavljeni uporabniki ali kdorkoli).

- Komentarji z anketo jo prikažejo pod besedilom. Bralci glasujejo s tapkanjem po možnosti, lahko prikažejo ali skrijejo rezultate pred glasovanjem in lahko spremenijo svoj glas. Rezultati se posodabljajo v živo, ko glasujejo drugi.
- Ankete se lahko zaprejo na datum, skrijejo rezultate do glasovanja in naredijo glasovalo vidne skrbnikom ali vsem ("Poglej, kdo je glasoval").
- Ko si gledalec lahko ustvari ankete, prikaže vnos komentarja v `FastCommentsView` gumb za anketo, ki odpre graditelj anket. Anketa se objavi skupaj s komentarjem, zato je besedilo komentarja obvezno. Vnos v živem klepetu gumba ne prikaže, tako kot spletni živi klepet.

Glasovanje zahteva prijavljenega uporabnika, zato uporabite [SSO](#authentication-sso-readme-generated). Strežnik poveže anonimni glas s piškotkom seje, ki ga odjemalec API ne shranjuje, zato pri `pollVoting`, nastavljenem na kdorkoli, gledalec brez uporabnika SSO vidi "Prijavite se za glasovanje". Ankete, ki skrijejo rezultate do glasovanja, ne dobijo posodobitev rezultatov v živo; ti glasovalci vidijo nove številke ob naslednjem nalaganju.

Za objavo ankete iz lastnega vmesnika podajte `CommentPollInput` funkciji `postComment`:

```swift
let draft = PollDraft(question: "Where should we eat?", options: ["Tacos", "Ramen"], closesDays: 1)
if draft.validate() == nil {
    try await sdk.postComment(text: "Lunch vote", poll: draft.toInput())
}
```

`sdk.votePoll(commentId:optionId:)` in `sdk.getPollVoters(commentId:page:search:)` pokrivata glasovanje in seznam glasovalcev. Barve anket so navedene pod [Barve anket](#theming-readme-generated).