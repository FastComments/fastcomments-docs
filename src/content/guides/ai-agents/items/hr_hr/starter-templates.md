FastComments isporučuje pet početnih predložaka tako da ne morate pisati radnog agenta od nule. Možete im pristupiti sa [stranice AI agenata](https://fastcomments.com/auth/my-account/ai-agents) klikom na **Pregledaj predloške**.

Kada odaberete predložak:

1. Agent se stvara sa **Status: Dry Run** i internim imenom zasnovanim na predlošku (`tos_enforcer`, `welcome_greeter`, `top_comment_pinner`, `thread_summarizer`, `gaslight_detector`). Ako je to ime već zauzeto na vašem tenantu, dodaje se numerički sufiks.
2. Odmah dolazite na obrazac za uređivanje s unaprijed popunjenim podacima - prompt, okidači, dopuštene radnje i svi pragovi. Na vrhu se prikazuje natpis "Stvoren iz predloška {templateName}. Pregledajte postavke u nastavku, zatim promijenite status u Enabled kad budete spremni."
3. Još ništa nije omogućeno. Agent neće djelovati dok ne spremite i ili ostavite dry-run uključen (za promatranje) ili prebacite na Enabled.

### Pet predložaka

- **[Moderator](#template-moderator)** - pregledava nove i označene komentare, upozorava prekršitelje koji to rade prvi put, eskalira do zabrane tek nakon upozorenja. Aktivira se na nove komentare i pri prelasku praga oznaka (zadani prag oznaka: 3). Dopušteni alati: `mark_comment_approved`, `mark_comment_spam`, `warn_user`, `ban_user`.

- **[Welcome Greeter](#template-welcome-greeter)** - srdačno odgovara korisnicima koji komentiraju prvi put kratkom, osobnom dobrodošlicom. Aktivira se na new-user-first-comment. Dopušteni alat: `write_comment`.

- **[Top Comment Pinner](#template-top-comment-pinner)** - prikvači sadržajne komentare na vrhu konverzacije kada prijeđu prag glasova (zadano: 10), prethodno otkvačivši prethodno prikvačeni komentar. Aktivira se pri prelasku praga glasova. Dopušteni alati: `pin_comment`, `unpin_comment`.

- **[Thread Summarizer](#template-thread-summarizer)** - objavljuje neutralni sažetak u jednom odlomku na dugim nitima nakon odgode, a zatim ga prikvači. Aktivira se na nove komentare s odgodom od 30 minuta kako bi se nit smirila prije sažimanja. Dopušteni alati: `write_comment`, `pin_comment`, `unpin_comment`.

- **[Gaslight Detector](#template-gaslight-detector)** - prati uređivanja komentara zbog prepravki u sredini niti koje iskrivljuju odgovore, vraća originalni tekst i šalje DM autoru. Aktivira se na uređivanja komentara. Dopušteni alati: `edit_comment`, `warn_user`, `send_dm`.

### Prilagodba predloška

Predlošci su polazne točke, a ne obveze. Očekuje se da ćete:

- Prilagoditi **Početni prompt** tako da odgovara tonu vaše zajednice.
- Dodati ili ukloniti **Okidače** kako bi odgovarala učestalost pokretanja agenta.
- Dodati **Odobrenja** za svaku osjetljivu radnju - snažno preporučujemo da `ban_user` zahtijeva odobrenje za predloške u stilu moderatora.
- Dodati **Smjernice zajednice** kako bi agent dosljedno primjenjivao vašu pisanu politiku. Vidi [Smjernice zajednice](#community-guidelines).
- Postaviti po-agentne **Budžete** odgovarajuće za očekivani broj okidača.

Predložak je samo sredstvo koje unaprijed popunjava razumna zadana podešavanja; nakon spremanja, agent je vaš.