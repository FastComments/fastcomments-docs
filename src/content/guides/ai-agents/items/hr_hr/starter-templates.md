FastComments dolazi s četiri početna predloška kako ne biste morali pisati radnog agenta od nule. Do njih možete doći preko [Stranica AI agenata](https://fastcomments.com/auth/my-account/ai-agents) klikom na **Browse templates**.

Kada odaberete predložak:

1. Agent se kreira sa **Status: Suhi način** i internim imenom temeljеним na predlošku (`tos_enforcer`, `welcome_greeter`, `top_comment_pinner`, `thread_summarizer`). Ako je to ime već zauzeto na vašem tenantu, dodaje se numerički sufiks.
2. Dolazite izravno na obrazac za uređivanje sa svime unaprijed popunjenim - promptom, okidačima, dozvoljenim akcijama i svim pragovima. Na vrhu se prikazuje banner s natpisom "Kreirano iz predloška {templateName}. Pregledajte postavke u nastavku, zatim promijenite status u Omogućeno kada budete spremni."
3. Još ništa nije omogućeno. Agent neće djelovati dok ne spremite i ili zadržite suhi način uključеним (za promatranje) ili ga prebacite na Omogućeno.

### Četiri predloška

- **[Moderator](#template-moderator)** - pregledava nove i označene komentare, upozorava korisnike koji prvi put počine prekršaj, eskalira na zabranu tek nakon upozorenja. Triggers on new comments and on flag-threshold crossings (default flag threshold: 3). Dozvoljeni alati: `mark_comment_approved`, `mark_comment_spam`, `warn_user`, `ban_user`.

- **[Welcome Greeter](#template-welcome-greeter)** - srdačno odgovara prvima koji komentiraju s kratkom, osobnom porukom dobrodošlice. Triggers on new-user-first-comment. Dozvoljeni alat: `write_comment`.

- **[Top Comment Pinner](#template-top-comment-pinner)** - prikvači sadržajne komentare na vrhu kada prijeđu vote threshold (default: 10), pri čemu prvo otkači prethodno prikvačeni komentar. Triggers on vote-threshold crossings. Dozvoljeni alati: `pin_comment`, `unpin_comment`.

- **[Thread Summarizer](#template-thread-summarizer)** - objavljuje neutralni, jednoparagrafski sažetak na dugim nitima nakon odgode, a zatim ga prikvači. Pokreće se na nove komentare s odgodom od 30 minuta kako bi se nit smirila prije sažimanja. Dozvoljeni alati: `write_comment`, `pin_comment`, `unpin_comment`.

### Prilagodba predloška

Predlošci su polazne točke, a ne ugovori. Od vas se očekuje da:

- Prilagodite **Initial prompt** kako bi odgovarao tonu vaše zajednice.
- Dodate ili uklonite **Triggers** kako biste prilagodili koliko često agent treba raditi.
- Dodate **Approvals** za bilo koju osjetljivu akciju - toplo preporučujemo da `ban_user` stavite iza odobrenja za predloške u stilu moderatora.
- Dodate **Community guidelines** kako bi agent dosljedno primjenjivao vašu pisanu politiku. Vidi [Smjernice zajednice](#community-guidelines).
- Postavite po-agentne **Budgets** odgovarajuće broju okidača koje očekujete.

Predložak je samo sredstvo koje unaprijed popunjava razumne zadane vrijednosti; nakon spremanja, agent je vaš.

---