Da vtičnik deluje, je žeton shranjen v vaši WordPress bazi podatkov in tudi v vašem FastComments računu. Ko vtičnik pošlje zahtevo na naše strežnike, posreduje
ta žeton.

Vse integracije, pooblaščene za vaš FastComments račun, si lahko ogledate [tukaj](https://fastcomments.com/auth/my-account/manage-data/integrations).

Vsa komunikacija poteka prek HTTPS.

Vsa komunikacija je *odhodna* z vašega WordPress strežnika *do* FastComments.com, vključno s sinhronizacijo *nazaj* v vašo namestitev WordPress, saj je izvedena
preko [polling](https://en.wikipedia.org/wiki/Polling_(computer_science)) iz [cron](https://developer.wordpress.org/plugins/cron/) nastavitve v vaši WordPress namestitvi.