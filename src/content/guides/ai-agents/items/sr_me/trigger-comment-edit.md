Pokreće agenta kada je komentar izmenjen.

### Kontekst koji agent prima

- Komentar u svom trenutnom (nakon izmjene) obliku.
- Tekst **prethodnog komentara** kao poseban ograničeni blok (`PREVIOUS_TEXT`). Ovo je jedinstveno za okidač izmjene - agent može uporediti prije/nakon.
- Opcionalna istorija teme / korisnika / kontekst stranice prema konfiguraciji.

### Napomene

- Okidač se aktivira za svaku uspješnu izmjenu, uključujući izmjene koje su izvršili moderatori u ime korisnika.
- Agentima nije izložen alat za uređivanje komentara; agenti uopšte ne mogu uređivati komentare.
- Tekst prethodnog komentara je označen kao nepouzdan unos. Sistemski prompt platforme podsjeća model da ne postupa po uputama koje se nalaze unutar takvih ograda — ovo je važno ovdje, jer zlonamjerni korisnik može izmijeniti komentar kako bi ubacio payload "ignoriši svoje prethodne upute" usmjeren na bilo kojeg agenta koji prati događaje izmjene.

### Uobičajena upotreba

- **Otkrivanje prerađenog sadržaja** - korisnik izmijeni prethodno čist komentar kako bi ubacio spam nakon što je moderator nastavio dalje.
- **Praćenje sitnih izmjena** - ako vaša zajednica tretira izmjene kao odvojene događaje iz bilo kojeg razloga za reviziju.

### Napomena o troškovima

Okidači izmjena vide dvije kopije teksta komentara (nova verzija u standardnom COMMENT bloku, stara verzija u PREVIOUS_TEXT bloku). Za duge komentare to otprilike udvostručuje troškove tokena pokretanja u odnosu na okidač `COMMENT_ADD` - imajte to na umu prilikom budžetiranja.