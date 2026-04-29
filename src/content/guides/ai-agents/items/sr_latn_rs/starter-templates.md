FastComments dolazi sa četiri početna šablona tako da ne morate pisati funkcionalnog agenta od nule. Možete im pristupiti sa [Stranica AI agenata](https://fastcomments.com/auth/my-account/ai-agents) klikom na **Browse templates**.

Kada odaberete šablon:

1. Agent se kreira sa **Status: Dry Run** i internim imenom zasnovanim na šablonu (`tos_enforcer`, `welcome_greeter`, `top_comment_pinner`, `thread_summarizer`). Ako je to ime već zauzeto na vašem tenant-u, dodeli se numerički sufiks.
2. Dolazite direktno na obrazac za uređivanje sa svime unapred popunjenim - prompt, okidači, dozvoljene radnje i svi pragovi. Baner na vrhu glasi "Created from the {templateName} template. Review the settings below, then change status to Enabled when you're ready."
3. Ništa još nije omogućeno. Agent neće delovati dok ne sačuvate i ili ostavite dry-run uključen (za posmatranje) ili ga prebacite na Enabled.

### The four templates

- **[Moderator](#template-moderator)** - pregledava nove i prijavljene komentare, upozorava prekršioce koji prvi put greše, eskalira do banovanja samo nakon upozorenja. Aktivira se na nove komentare i na prelazak praga prijava (default flag threshold: 3). Dozvoljeni alati: `mark_comment_approved`, `mark_comment_spam`, `warn_user`, `ban_user`.

- **[Welcome Greeter](#template-welcome-greeter)** - srdačno odgovara korisnicima koji prvi put komentarišu kratkom, ličnom porukom dobrodošlice. Aktivira se na new-user-first-comment. Dozvoljeni alat: `write_comment`.

- **[Top Comment Pinner](#template-top-comment-pinner)** - zakači značajne komentare vršnog nivoa kada pređu prag glasova (podrazumevani prag: 10), prethodno otkačivši ranije zakačeni komentar. Aktivira se na prelazak praga glasova. Dozvoljeni alati: `pin_comment`, `unpin_comment`.

- **[Thread Summarizer](#template-thread-summarizer)** - postavlja neutralan, jednoparagrafni rezime na dugim temama nakon odlaganja, zatim ga zakači. Aktivira se na nove komentare sa odlaganjem od 30 minuta kako bi se tema slegla pre sumiranja. Dozvoljeni alati: `write_comment`, `pin_comment`, `unpin_comment`.

### Customizing a template

Šabloni su polazne tačke, a ne ugovori. Od vas se očekuje da:

- Doradite **Initial prompt** da odgovara glasu vaše zajednice.
- Dodate ili uklonite **Triggers** kako bi odgovaralo koliko često agent treba da radi.
- Dodate **Approvals** za svaku osetljivu radnju - snažno preporučujemo da postavite `ban_user` iza odobrenja za šablone u stilu moderatora.
- Dodate **Community guidelines** tako da agent dosledno primenjuje vašu pisanu politiku. Pogledajte [Community Guidelines](#community-guidelines).
- Podesite po-agentne **Budžete** u skladu sa očekivanim brojem okidača.

Šablon je samo sredstvo koje predpopunjava smisleno podrazumevana podešavanja; kada ga sačuvate, agent je vaš.