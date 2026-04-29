Aktivira agenta kada je komentar uređen.

### Kontekst koji agent prima

- Komentar u svom trenutnom (nakon uređivanja) obliku.
- Tekst **prethodnog komentara** kao zaseban oivičeni blok (`PREVIOUS_TEXT`). Ovo je jedinstveno za okidač uređivanja - agent može usporediti prije/nakon.
- Neobavezna povijest rasprave / korisnika / kontekst stranice prema konfiguraciji.

### Napomena

- Okidač se pokreće za svako uspješno uređivanje, uključujući uređivanja koja su obavili moderatori u ime korisnika.
- Agentima nije dostupan alat za uređivanje komentara; agenti uopće ne mogu uređivati komentare.
- Tekst prethodnog komentara je oivičen kao nepouzdani ulaz. Sistemski prompt platforme podsjeća model da ne slijedi upute iz unutar oivičenja - ovo je važno ovdje, jer bi zlonamjerni korisnik mogao urediti komentar kako bi umetnuo teret "zanemarite svoje prethodne upute" usmjeren prema bilo kojem agentu koji prati događaje uređivanja.

### Uobičajene upotrebe

- **Otkrivanje prikrivenog sadržaja** - korisnik uređuje prethodno čist komentar da bi ubacio spam nakon što je moderator otišao.
- **Praćenje manjih izmjena** - ako vaša zajednica tretira izmjene kao zasebne događaje iz bilo kojeg razloga za reviziju.

### Napomena o troškovima

Okidači uređivanja vide dvije kopije teksta komentara (nova verzija u standardnom COMMENT bloku, stara verzija u PREVIOUS_TEXT bloku). Za duge komentare to otprilike udvostručuje trošak tokena izvođenja u usporedbi s `COMMENT_ADD` okidačem - imajte to na umu prilikom planiranja proračuna.