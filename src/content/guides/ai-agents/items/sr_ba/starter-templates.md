FastComments dolazi sa pet početnih predložaka tako da ne morate pisati funkcionalnog agenta od nule. Dostupni su sa [Stranice AI agenata](https://fastcomments.com/auth/my-account/ai-agents) klikom na **Pregled predložaka**.

Kada odaberete predložak:

1. Agent se kreira sa **Status: Dry Run** i internim imenom baziranim na predlošku (`tos_enforcer`, `welcome_greeter`, `top_comment_pinner`, `thread_summarizer`, `gaslight_detector`). Ako je to ime već zauzeto na vašem tenant-u, dodaje se numerički sufiks.
2. Dolazite direktno na obrazac za uređivanje sa svime unaprijed popunjenim - promptom, okidačima, dozvoljenim akcijama i svim pragovima. Banner preko vrha prikazuje "Created from the {templateName} template. Review the settings below, then change status to Enabled when you're ready."
3. Još ništa nije omogućeno. Agent neće djelovati dok ne sačuvate i ili ostavite dry-run uključen (za promatranje) ili prebacite na Enabled.

### Pet predložaka

- **[Moderator](#template-moderator)** - pregleda nove i prijavljene komentare, upozorava prve prekršioce, i eskalira do zabrane samo nakon upozorenja. Okida na nove komentare i na flag-threshold crossings (zadani prag za prijave: 3). Dozvoljeni alati: `mark_comment_approved`, `mark_comment_spam`, `warn_user`, `ban_user`.

- **[Welcome Greeter](#template-welcome-greeter)** - srdačno odgovara prvima koji komentarišu sa kratkim, ličnim pozdravom. Okida na new-user-first-comment. Dozvoljeni alat: `write_comment`.

- **[Top Comment Pinner](#template-top-comment-pinner)** - prikvači značajne komentare na vrhu kada pređu prag glasova (zadano: 10), prvo odklanjajući prethodno prikvačeni komentar. Okida na vote-threshold crossings. Dozvoljeni alati: `pin_comment`, `unpin_comment`.

- **[Thread Summarizer](#template-thread-summarizer)** - objavljuje neutralan, jednoparagrafni sažetak na dugim temama nakon odgode, a zatim ga prikvači. Okida na nove komentare sa 30-minutnom odgodom kako bi se tema smirila prije sažimanja. Dozvoljeni alati: `write_comment`, `pin_comment`, `unpin_comment`.

- **[Gaslight Detector](#template-gaslight-detector)** - prati uređivanja komentara radi preinaka u toku teme koje iskrivljuju odgovore, vraća originalni tekst i šalje DM autoru. Okida na uređivanja komentara. Dozvoljeni alati: `edit_comment`, `warn_user`, `send_dm`.

### Prilagođavanje predloška

Predlošci su početne tačke, a ne ugovori. Očekuje se da:

- Podesite **Initial prompt** da odgovara glasu vaše zajednice.
- Dodajte ili uklonite **Triggers** kako bi se poklapalo sa učestalošću rada agenta.
- Dodajte **Approvals** za bilo koju osjetljivu akciju - snažno preporučujemo stavljanje `ban_user` iza odobrenja za predloške u stilu moderatora.
- Dodajte **Community guidelines** kako bi agent dosljedno primjenjivao vašu pisanu politiku. Pogledajte [Community Guidelines](#community-guidelines).
- Postavite za svaki agent **Budgets** u skladu s brojem okidača koje očekujete.

Predložak je samo sredstvo koje unaprijed popunjava razumna zadana podešavanja; nakon spremanja, agent je vaš.