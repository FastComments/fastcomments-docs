Pokreće agenta kada se komentar izmijeni.

### Kontekst koji agent prima

- Komentar u njegovom trenutnom (nakon izmjene) obliku.
- Tekst **prethodnog komentara** kao zaseban ograđeni blok (`PREVIOUS_TEXT`). Ovo je specifično za okidač izmjene - agent može uporediti prije/nakon.
- Opcionalna nit / istorija korisnika / kontekst stranice kako je konfigurirano.

### Napomene

- Okidač se aktivira za svaku uspješnu izmjenu, uključujući izmjene koje moderatori izvrše u ime korisnika.
- Agentima nije izložen alat za uređivanje komentara; agenti uopće ne mogu uređivati komentare.
- Tekst prethodnog komentara je ograđen kao nepouzdan ulaz. Sistem prompt platforme podsjeća model da ne slijedi upute koje se nalaze unutar ograda - ovo je važno zato što zlonamjeran korisnik može urediti komentar kako bi ubacio sadržaj "ignoriši svoje prethodne instrukcije" usmjeren prema bilo kojem agentu koji prati događaje izmjena.

### Uobičajene upotrebe

- **Otkrivanje prikrivenog sadržaja** - korisnik uređuje prethodno čist komentar kako bi ubacio spam nakon što je moderator prešao dalje.
- **Praćenje manjih izmjena** - ako vaša zajednica tretira izmjene kao odvojene događaje iz bilo kojeg razloga za reviziju.

### Napomena o troškovima

Okidači za izmjene vide dvije kopije teksta komentara (nova verzija u standardnom `COMMENT` bloku, stara verzija u `PREVIOUS_TEXT` bloku). Za duge komentare ovo otprilike udvostručuje broj tokena po izvršavanju u odnosu na `COMMENT_ADD` okidač - imajte to na umu prilikom planiranja budžeta.