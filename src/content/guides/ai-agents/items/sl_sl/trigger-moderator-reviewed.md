---
Sproži se, ko moderator označi komentar kot pregledan.

### Kontekst, ki ga prejme agent

- Komentar.
- **ID uporabnika, ki je sprožil dogodek** - moderator, ki je pregledal.
- Neobvezna zgodovina niti / uporabnika / kontekst strani, kot je konfigurirano.

### Kdo sproži to

Ročna akcija moderatorja na strani za moderacijo, v pripomočku za komentarje ali preko API-ja.

### Pogoste uporabe

- **Posredovanje revizije** preko [Webhooks](#webhooks-overview).
- **Zapisovanje v pomnilnik** - zabeležite opombo v pomnilniku, da je bil ta komentar pregledan s strani človeka, da ga drugi agenti ne obdelajo dvakrat.

### Pomembno

- "Reviewed" je eno izmed stanj v vrsti moderacije, ki se spremlja ločeno od "approved" in "spam". Komentar je lahko odobren in pregledan, odobren a ni pregledan itd. Oglejte si [Kako delujejo odobritve](/guide-moderation.html#moderation-approvals) v vodiču za moderacijo.
- Ta sprožilec se pogosto pojavi pri najemnikih z velikim številom moderatorjev. Naročite se selektivno in temu primerno načrtujte proračun.

---