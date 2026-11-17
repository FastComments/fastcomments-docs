Ankete delujejo enako kot v spletnem gradniku in vklopijo se iz istih nastavitev gradnika v vaši FastComments nadzorni plošči: `pollCreation` (kdo lahko priloži anketo: onemogočeno, skrbniki in moderatorji ali vsi) in `pollVoting` (prijavljeni uporabniki ali kdorkoli).

- Komentarji z anketo jo prikažejo pod svojim besedilom. Bralci glasujejo s tapkanjem po možnosti, lahko pred glasovanjem prikažejo ali skrijejo rezultate in lahko spremenijo svoj glas. Rezultati se posodabljajo v živo, ko glasujejo drugi.
- Ankete se lahko zaprejo na datum, skrijejo rezultate do glasovanja in naredijo glasovalce vidne skrbnikom ali vsem ("Poglej, kdo je glasoval").
- Ko si gledalec sme ustvariti ankete, vnos komentarja v `FastCommentsView` prikaže gumb za anketo, ki odpre graditelja anket. Anketa se objavi skupaj s komentarjem, zato je besedilo komentarja obvezno. Vnos v živem klepetu gumba ne prikaže, tako kot spletni živi klepet.

Glasovanje zahteva prijavljenega uporabnika, zato uporabite [SSO](#authentication-sso-readme-generated). Strežnik poveže anonimni glas s piškotkom seje, ki ga odjemalec API ne hrani, tako da je `pollVoting` nastavljen na kdorkoli, gledalec brez SSO uporabnika vidi "Prijavite se za glasovanje". Ankete, ki skrijejo rezultate do glasovanja, ne prejemajo posodobitev rezultatov v živo; ti glasovalci vidijo nove številke ob naslednjem nalaganju.

Za objavo ankete iz lastnega vmesnika podajte `CommentPollInput` funkciji `postComment`:

```swift
let draft = PollDraft(question: "Where should we eat?", options: ["Tacos", "Ramen"], closesDays: 1)
if draft.validate() == nil {
    try await sdk.postComment(text: "Lunch vote", poll: draft.toInput())
}
```

`sdk.votePoll(commentId:optionId:)` in `sdk.getPollVoters(commentId:page:search:)` pokrivata glasovanje in seznam glasovalcev. Barve anket so navedene pod [Barve anket](#poll-colors-readme-generated).