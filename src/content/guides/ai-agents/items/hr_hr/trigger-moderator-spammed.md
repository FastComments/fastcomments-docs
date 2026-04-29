Pokreće se kada moderator označi komentar kao spam.

### Kontekst koji agent prima

- Komentar, s post-akcijskom zastavicom `Is Spam`.
- **ID korisnika koji je pokrenuo događaj** - moderator koji je djelovao.
- Opcionalni kontekst teme / povijesti korisnika / stranice prema konfiguraciji.

### Tko pokreće ovo

Akcija ljudskog moderatora. Oznake spama koje dolaze od agenta (preko [`mark_comment_spam`](#tools-overview)) **ne** pokreću ovaj okidač.

### Česte upotrebe

- **Zapisivanje memorije** - neka agent pohrani bilješku u memoriju o korisniku označenom kao spam (npr. "ranije označen kao spam zbog X od strane moderatora") kako bi budući agenti za moderaciju imali kontekst.
- **Provedba na razini korisnika** - moderator koji označi komentar kao spam može biti povod da agent također izda upozorenje ili kratku suspenziju, uz odobrenje.
- **Zrcaljenje vanjskog sustava** putem [Webhooks](#webhooks-overview).

---