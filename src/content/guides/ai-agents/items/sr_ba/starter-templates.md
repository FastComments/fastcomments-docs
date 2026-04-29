FastComments dolazi sa četiri početna predloška kako ne biste morali pisati radnog agenta od nule. Do njih se može pristupiti sa [AI Agents page](https://fastcomments.com/auth/my-account/ai-agents) klikom na **Pregled predložaka**.

When you pick a template:

1. Agent se kreira sa **Status: Dry Run** i internim imenom zasnovanim na predlošku (`tos_enforcer`, `welcome_greeter`, `top_comment_pinner`, `thread_summarizer`). Ako je to ime već zauzeto na vašem tenant-u, dodaje se numerički sufiks.
2. Odmah se nađete na formi za uređivanje sa svime unaprijed popunjeno - prompt, triggers, allowed actions, and any thresholds. Baner preko vrha glasi "Created from the {templateName} template. Review the settings below, then change status to Enabled when you're ready."
3. Ništa još nije omogućeno. Agent neće djelovati dok ne sačuvate i ili ostavite dry-run uključen (za posmatranje) ili ga prebacite na Enabled.

### Četiri predloška

- **[Moderator](#template-moderator)** - pregledava nove i prijavljene komentare, upozorava počinitelje prvi put, eskalira do zabrane tek nakon upozorenja. Okida na nove komentare i na prelazak praga prijava (zadani prag prijava: 3). Dozvoljeni alati: `mark_comment_approved`, `mark_comment_spam`, `warn_user`, `ban_user`.

- **[Welcome Greeter](#template-welcome-greeter)** - srdačno odgovara komentatorima koji prvi put komentarišu kratkom, personalnom porukom dobrodošlice. Okida na new-user-first-comment. Dozvoljen alat: `write_comment`.

- **[Top Comment Pinner](#template-top-comment-pinner)** - zakači sadržajne top-level komentare kada pređu prag glasova (zadano: 10), prije toga otkači prethodno zakačeni komentar. Okida na prelazak praga glasova. Dozvoljeni alati: `pin_comment`, `unpin_comment`.

- **[Thread Summarizer](#template-thread-summarizer)** - objavljuje neutralan, jednoparagrafni sažetak na dugim nitima nakon odgode, zatim ga zakači. Okida na nove komentare sa odgodom od 30 minuta kako bi se nit smirila prije sažimanja. Dozvoljeni alati: `write_comment`, `pin_comment`, `unpin_comment`.

### Prilagođavanje predloška

Predlošci su polazne tačke, ne ugovori. Očekuje se da ćete:

- Prilagoditi **Initial prompt** da odgovara glasu vaše zajednice.
- Dodati ili ukloniti **Triggers** da odgovara učestalosti rada agenta.
- Dodati **Approvals** za svaku osjetljivu akciju - toplo preporučujemo da `ban_user` stavite pod odobrenje za predloške tipa moderator.
- Dodati **Community guidelines** kako bi agent dosljedno primjenjivao vašu pisanu politiku. Pogledajte [Community Guidelines](#community-guidelines).
- Postaviti po-agentu **Budgets** odgovarajuće broju okidača koje očekujete.

Predložak je samo vozilo koje unaprijed popunjava razumna zadana podešavanja; kada ga sačuvate, agent je vaš.