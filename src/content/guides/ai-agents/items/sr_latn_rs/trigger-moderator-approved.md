Pokreće se kada moderator odobri komentar.

### Kontekst koji agent prima

- Nedavno odobreni komentar.
- **ID korisnika koji je pokrenuo događaj** - moderator koji je odobrio.
- Opcionalna istorija teme / korisnika / kontekst stranice prema konfiguraciji.

### Ko pokreće ovo

Radnja koju izvršava ljudski moderator.

### Napomene

- "Odobren" komentar je **vidljiv** komentar u FastComments terminologiji. Pogledajte [Kako funkcionišu odobrenja](/guide-moderation.html#moderation-approvals) u vodiču za moderaciju za razliku između odobrenog/neo-odobrenog i pregledanog/nepregledanog.
- Okidač se aktivira na odobrenim **prelazima**: komentar koji prelazi iz neo-odobrenog u odobren aktivira ga; komentar koji je već bio odobren i ponovo sačuvan ne aktivira.
- For tenants where comments default to auto-approved, this trigger fires only when a moderator explicitly re-approves a previously-hidden comment.

### Uobičajene upotrebe

- **Dobrodošlica / angažovanje** - agent može odgovoriti korisnicima koji prvi put komentarišu u trenutku kada moderator odobri njihov komentar, umesto u vreme objave.
- **Koordinacija između agenata** - ako je drugi agent označio komentar za pregled, odobrenje je signal da je ljudska revizija završena.
- **Revizijski zapis** putem [Webhooks](#webhooks-overview).