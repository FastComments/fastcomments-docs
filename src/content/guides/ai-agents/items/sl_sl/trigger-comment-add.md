---
Sproži agenta vsakič, ko je na strani, ki jo pokriva agentov [obseg](#scope-url-locale), objavljen nov komentar.

### Kontekst, ki ga agent prejme

- Celoten nov komentar - besedilo, avtor, glasovi, ID nadrejenega, ID URL strani.
- Izbirno: nadrejeni komentar in prejšnji odgovori v isti niti, če je [kontekst niti](#context-options) vklopljen.
- Izbirno: zaupanja vrednost komentatorja, starost računa, zgodovina prepovedi in nedavni komentarji, če je vklopljen [kontekst uporabnikove zgodovine](#context-options).
- Izbirno: metapodatki strani, če je vklopljen [kontekst strani](#context-options).

### Pomembno

- Sprožilec se sproži **po tem**, ko je komentar shranjen. Agent se lahko nanj neposredno sklicuje v klicih orodij.
- Sprožilec se **ne** sproži za komentarje, ki jih je napisal drug agent v istem najemniku.
- Sproži se za tako preverjene kot nepreverjene komentarje. Če vaš najemnik zahteva odobritev moderatorja, preden je komentar viden (glejte [Kako delujejo odobritve](/guide-moderation.html#moderation-approvals) v vodiču za moderacijo), se sprožilec sproži ob ustvarjanju komentarja, ne ob kasnejši odobritvi. Moderatorskemu botu je mogoče po pregledu naročiti, naj za vas odobri komentarje.

### Pogoste uporabe

- **Moderacija** - preverite komentar glede na smernice skupnosti, označite kot neželeno pošto ali opozorite tiste, ki komentirajo prvič.
- **Pozdrav ob prihodu** - čeprav je [Sprožilec: Prvi komentar novega uporabnika](#trigger-new-user-first-comment) običajno bolj primeren za pozdrave, saj se sproži enkrat na uporabnika.
- **Povzemanje niti** - običajno v povezavi z [zamikom sprožilca](#trigger-deferred-delay), da se nit umiri, preden agent zažene.
---