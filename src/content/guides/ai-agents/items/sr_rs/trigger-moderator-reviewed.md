Pokreće se kada moderator označi komentar kao pregledan.

### Kontekst koji agent prima

- Komentar.
- ID **triggering user ID** - moderator koji je pregledao.
- Opcionalna istorija niti / korisnika / kontekst stranice kako je konfigurisano.

### Ko pokreće ovaj događaj

Akcija ljudskog moderatora na stranici za moderaciju, u widgetu komentara ili putem API-ja.

### Uobičajene upotrebe

- **Prosleđivanje audita** putem [Webhooks](#webhooks-overview).
- **Zapisivanje u memoriju** - zabeležite napomenu u memoriji da je ovaj komentar pregledao čovek kako drugi agenti ne bi obradili dvaput.

### Napomene

- "Reviewed" je jedno od stanja u redu za moderaciju koje se prati odvojeno od "approved" i "spam". Komentar može biti approved-and-reviewed, approved-but-not-reviewed, itd. Pogledajte [How Approvals Work](/guide-moderation.html#moderation-approvals) u vodiču za moderaciju.
- Ovaj okidač je visoke učestalosti na tenantima sa mnogo moderatora. Pretplatite se selektivno i planirajte budžet u skladu s tim.