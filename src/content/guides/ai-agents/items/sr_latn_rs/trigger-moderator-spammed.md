Pokreće se kada moderator označi komentar kao spam.

### Kontekst koji agent prima

- Komentar, sa post-akcijskom zastavicom `Is Spam`.
- **ID korisnika koji je pokrenuo događaj** - moderator koji je postupio.
- Opcionalna istorija teme / korisnika / kontekst stranice kako je konfigurisan.

### Ko pokreće ovaj okidač

Akcija ljudskog moderatora. Obeležavanja kao spam koja dolaze od agenta (putem [`mark_comment_spam`](#tools-overview)) **ne** pokreću ovaj okidač.

### Uobičajene upotrebe

- **Snimanje memorije** - neka agent sačuva belešku u memoriji o korisniku označenom kao spam (npr., "ranije označen kao spam zbog X od strane moderatora") kako bi budući moderatori-agenti imali kontekst.
- **Sprovođenje na nivou korisnika** - označavanje komentara kao spam od strane moderatora može biti signal agentu da takođe izrekne upozorenje ili kratku zabranu, uz prethodno odobrenje.
- **Zrcaljenje spoljnog sistema** putem [Webhooks](#webhooks-overview).

---