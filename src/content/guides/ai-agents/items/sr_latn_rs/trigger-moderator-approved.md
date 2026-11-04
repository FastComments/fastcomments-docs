Aktivira se kada moderator odobri komentar.

### Kontekst koji agent prima

- Novoodobreni komentar.
- **ID korisnika koji je okinuo događaj** - moderator koji je odobrio.
- Opcioni kontekst niti / istorije korisnika / stranice, prema konfiguraciji.

### Ko ovo okida

Akacija ljudskog moderatora.

### Napomene

- "Odobren" komentar je **vidljiv** komentar u FastComments terminologiji. Pogledajte [Kako funkcionišu odobrenja](/guide-moderation.html#moderation-approvals) u vodiču za moderaciju za razliku između odobreno/neodobreno i pregledano/nepregledano.
- Okidač se aktivira pri **prelazima** odobrenja: komentar koji prelazi iz neodobrenog u odobreni ga okida; komentar koji je već bio odobren i ponovo se čuva ga ne okida.
- Za tenant-ove gde se komentari podrazumevano automatski odobravaju, ovaj okidač se aktivira samo kada moderator eksplicitno ponovo odobri ranije skriveni komentar.

### Uobičajene upotrebe

- **Dobrodošlica / angažovanje** - agent može odgovoriti prvim komentatorima u trenutku kada moderator odobri njihov komentar, umesto u trenutku objavljivanja.
- **Koordinacija između agenata** - ako je drugi agent označio komentar za pregled, odobrenje je signal da je ljudski pregled završen.
- **Revizioni trag** putem [Webhook-ova](#webhooks-overview).