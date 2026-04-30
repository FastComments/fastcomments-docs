FastComments dolazi sa pet početnih šablona tako da ne morate pisati funkcionalnog agenta od nule. Do njih se može doći sa [stranice AI agenata](https://fastcomments.com/auth/my-account/ai-agents) klikom na **Pregled šablona**.

Kada izaberete šablon:

1. Agent se kreira sa **Status: Dry Run** i internim imenom zasnovanim na šablonu (`tos_enforcer`, `welcome_greeter`, `top_comment_pinner`, `thread_summarizer`, `gaslight_detector`). Ako je to ime zauzeto na vašem tenantu, dodaće se numerički sufiks.
2. Bićete prebačeni direktno na obrazac za uređivanje sa svime unapred popunjenim - promptom, okidačima, dozvoljenim akcijama i eventualnim pragovima. Baner preko vrha prikazuje "Kreirano iz šablona {templateName}. Pregledajte podešavanja ispod, a zatim promenite status u Enabled kada budete spremni."
3. Još ništa nije omogućeno. Agent neće delovati dok ne sačuvate i ili ostavite Dry Run uključen (da posmatrate) ili prebacite na Enabled.

### Pet šablona

- **[Moderator](#template-moderator)** - pregledava nove i prijavljene komentare, upozorava korisnike koji prvi put prekrše pravila, eskalira do banovanja samo nakon upozorenja. Okidači: novi komentari i prelazak praga prijavljivanja (podrazumevani prag prijavljivanja: 3). Dozvoljeni alati: `mark_comment_approved`, `mark_comment_spam`, `warn_user`, `ban_user`.

- **[Welcome Greeter](#template-welcome-greeter)** - odgovara srdačno prvim komentatorima sa kratkom, ličnom dobrodošlicom. Okida na new-user-first-comment. Dozvoljen alat: `write_comment`.

- **[Top Comment Pinner](#template-top-comment-pinner)** - zakači značajne komentare prvog nivoa kada pređu prag glasova (podrazumevano: 10), pri čemu prvo otkači prethodno zakačeni komentar. Okida na prelazak praga glasova. Dozvoljeni alati: `pin_comment`, `unpin_comment`.

- **[Thread Summarizer](#template-thread-summarizer)** - objavljuje neutralan, jednoparagrafski rezime na dugim nitima nakon odlaganja, a zatim ga zakači. Okida na nove komentare sa odlaganjem od 30 minuta kako bi se nit smirila pre sažimanja. Dozvoljeni alati: `write_comment`, `pin_comment`, `unpin_comment`.

- **[Gaslight Detector](#template-gaslight-detector)** - prati izmene komentara zbog prepravki u toku diskusije koje iskrivljuju odgovore, vraća originalni tekst i šalje DM autoru. Okida na izmene komentara. Dozvoljeni alati: `edit_comment`, `warn_user`, `send_dm`.

### Prilagođavanje šablona

Šabloni su početne tačke, a ne ugovori. Očekuje se da ćete:

- Podesite **Initial prompt** tako da odgovara glasu vaše zajednice.
- Dodajte ili uklonite **Triggers** da odgovaraju učestalosti pokretanja agenta.
- Dodajte **Approvals** za bilo koju osetljivu akciju - toplo preporučujemo da `ban_user` stavite iza odobrenja za šablone u stilu moderatora.
- Dodajte **Community guidelines** tako da agent dosledno primenjuje vašu pisanu politiku. Pogledajte [Smernice zajednice](#community-guidelines).
- Podesite po-agentu **Budgets** odgovarajuće očekivanom broju okidača.

Šablon je samo sredstvo koje unapred popunjava razumna podrazumevana podešavanja; nakon što ga sačuvate, agent je vaš.