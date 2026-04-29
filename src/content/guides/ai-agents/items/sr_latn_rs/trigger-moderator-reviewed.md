Okida se kada moderator označi komentar kao pregledan.

### Kontekst koji agent prima

- Komentar.
- **triggering user ID** - moderator koji je pregledao.
- Opcionalna istorija niti / korisnika / kontekst stranice kako je konfigurisan.

### Ko pokreće ovo

Radnja ljudskog moderatora na strani za moderaciju, u widgetu komentara, ili preko API-ja.

### Uobičajene upotrebe

- **Prosleđivanje revizije** preko [Webhooks](#webhooks-overview).
- **Memory writes** - zabeležite napomenu u memoriji da je ovaj komentar pregledao čovek kako drugi agenti ne bi duplirali obradu.

### Napomena

- "Reviewed" je jedno od stanja u redu za moderaciju koja se prate odvojeno od "approved" i "spam". Komentar može biti approved-and-reviewed, approved-but-not-reviewed, itd. Pogledajte [How Approvals Work](/guide-moderation.html#moderation-approvals) u vodiču za moderaciju.
- Ovaj okidač se često aktivira na tenantima sa mnogo moderatora. Pretplatite se selektivno i planirajte budžet u skladu s tim.

---