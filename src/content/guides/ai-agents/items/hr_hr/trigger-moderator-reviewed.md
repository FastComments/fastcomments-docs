---
Pokreće se kada moderator označi komentar kao pregledan.

### Kontekst koji agent prima

- Komentar.
- **ID korisnika koji je pokrenuo događaj** - moderator koji je pregledao.
- Opcionalna povijest niti / korisnička povijest / kontekst stranice prema konfiguraciji.

### Tko pokreće ovo

Radnja ljudskog moderatora na stranici za moderaciju, widgetu za komentare ili putem API-ja.

### Uobičajene upotrebe

- **Prosljeđivanje audita** putem [Webhooks](#webhooks-overview).
- **Zapis u memoriju** - zabilježite napomenu u memoriji da je ovaj komentar pregledao čovjek kako drugi agenti ne bi obrađivali istu stavku dvaput.

### Napomene

- "Pregledano" je jedno od stanja u redu za moderaciju koje se prati odvojeno od "odobreno" i "spam". Komentar može biti odobren-i-pregledan, odobren-ali-ne-pregledan, itd. Pogledajte [Kako funkcionira odobravanje](/guide-moderation.html#moderation-approvals) u vodiču za moderaciju.
- Ovaj okidač se često aktivira na tenantima s mnogo moderatora. Pretplatite se selektivno i planirajte budžet u skladu s tim.

---