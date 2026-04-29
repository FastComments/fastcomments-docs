Okida se kada moderator odobri komentar.

### Kontekst koji agent dobiva

- Nedavno odobreni komentar.
- **ID korisnika koji pokreće okidač** - moderator koji je odobrio.
- Opcionalno: kontekst rasprave / povijest korisnika / kontekst stranice, prema konfiguraciji.

### Tko pokreće ovo

Akcija ljudskog moderatora.

### Napomena

- "Odobreni" komentar je u terminologiji FastCommentsa **vidljiv** komentar. Pogledajte [Kako funkcioniraju odobrenja](/guide-moderation.html#moderation-approvals) u vodiču za moderiranje za razliku između odobrenih/neodobrenih i pregledanih/nepregledanih.
- Okidač se aktivira prilikom promjene odobrenja (**prijelazi**): komentar koji prelazi iz neodobrenog u odobren aktivira ga; komentar koji je već bio odobren i ponovno spremljen ne aktivira.
- Za tenantima u kojima su komentari prema zadanim postavkama automatski odobreni, ovaj okidač se aktivira samo kada moderator eksplicitno ponovno odobri prethodno skriveni komentar.

### Uobičajene upotrebe

- **Dobrodošlica / angažman** - agent može odgovoriti korisnicima koji komentiraju prvi put u trenutku kada ih moderator odobri, umjesto u trenutku objave.
- **Koordinacija među agentima** - ako je neki drugi agent označio komentar za pregled, odobrenje je znak da je ručni pregled završen.
- **Revizijski zapis** putem [Webhooks](#webhooks-overview).