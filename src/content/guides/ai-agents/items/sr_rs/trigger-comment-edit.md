Pokreće agenta kada se komentar izmeni.

### Kontekst koji agent prima

- Komentar u svom trenutnom (posle izmene) obliku.
- **prethodni tekst komentara** kao zaseban fenced blok (`PREVIOUS_TEXT`). Ovo je jedinstveno za okidač izmene - agent može da uporedi pre i posle.
- Opcionalna istorija teme / korisnika / stranice u zavisnosti od konfiguracije.

### Napomene

- Okidač se pokreće za svaku uspešnu izmenu, uključujući izmene koje su izvršili moderatori u ime korisnika.
- Agentima nije dostupan alat za izmenu komentara; agenti uopšte ne mogu da menjaju komentare.
- Prethodni tekst komentara je ograničen kao nepoveren ulaz. Sistem prompt platforme podseća model da ne sledi instrukcije iz unutrašnjosti ograda - ovo je važno jer zlonameran korisnik može izmeniti komentar da ubaci payload "ignore your previous instructions" usmeren na bilo kog agenta koji prati događaje izmena.

### Uobičajene upotrebe

- **Otkrivanje maskiranog sadržaja** - korisnik izmeni prethodno čist komentar da umetne spam nakon što se moderator ode.
- **Praćenje manjih izmena** - ako vaša zajednica tretira izmene kao zasebne događaje iz bilo kog revizijskog razloga.

### Napomena o troškovima

Edit triggers see two copies of the comment text (the new version in the standard COMMENT block, the old version in the PREVIOUS_TEXT block). For long comments this roughly doubles the token cost of the run vs. a `COMMENT_ADD` trigger - keep that in mind when budgeting.

---