Okida agenta kada se komentar izmeni.

### Kontekst koji agent prima

- Komentar u svojoj trenutnoj (nakon izmene) formi.
- **prethodni tekst komentara** kao poseban ograničeni blok (`PREVIOUS_TEXT`). Ovo je jedinstveno za okidač izmene - agent može uporediti pre/posle.
- Opcionalna istorija teme / korisnika / kontekst stranice prema konfiguraciji.

### Napomene

- Okidač se aktivira za svaku uspešnu izmenu, uključujući izmene koje su izvršili moderatori u ime korisnika.
- Agentima nije izložen alat za uređivanje komentara; agenti uopšte ne mogu da uređuju komentare.
- Prethodni tekst komentara je ograničen kao nepouzdan ulaz. Sistem prompt platforme podseća model da ne sledi uputstva iznutra ograda - ovo je bitno ovde, zato što zlonamerni korisnik može izmeniti komentar da ubaci payload "ignoriši svoja prethodna uputstva" koji je usmeren na bilo kojeg agenta koji prati događaje izmena.

### Uobičajene upotrebe

- **Otkrivanje maskiranog sadržaja** - korisnik izmeni prethodno čist komentar da ubaci spam nakon što je moderator već otišao.
- **Praćenje manjih izmena** - ako vaša zajednica tretira izmene kao odvojene događaje iz bilo kog razloga revizije.

### Napomena o troškovima

Okidači izmene vide dve kopije teksta komentara (nova verzija u standardnom COMMENT bloku, stara verzija u PREVIOUS_TEXT bloku). Za duge komentare ovo otprilike udvostručuje broj tokena po izvršenju u odnosu na `COMMENT_ADD` okidač - imajte to na umu prilikom planiranja budžeta.