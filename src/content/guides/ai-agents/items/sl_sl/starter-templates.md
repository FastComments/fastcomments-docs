FastComments ponuja pet začetnih predlog, tako da vam ni treba pisati delujočega agenta od začetka. Dostopne so s [strani AI Agents](https://fastcomments.com/auth/my-account/ai-agents) s klikom na **Browse templates**.

Ko izberete predlogo:

1. Agent je ustvarjen s **Status: Dry Run** in internim imenom, ki temelji na predlogi (`tos_enforcer`, `welcome_greeter`, `top_comment_pinner`, `thread_summarizer`, `gaslight_detector`). Če je to ime na vašem najemniku že zasedeno, se doda številčna pripona.
2. Neposredno pristanete na obrazcu za urejanje, kjer je vse vnaprej izpolnjeno - poziv, sprožilci, dovoljena dejanja in morebitni pragovi. Pasica na vrhu glasi "Created from the {templateName} template. Review the settings below, then change status to Enabled when you're ready."
3. Nič še ni omogočeno. Agent ne bo deloval, dokler ne shranite in bodisi pustite vklopljen suhi zagon (za opazovanje) bodisi preklopite na Enabled.

### Pet predlog

- **[Moderator](#template-moderator)** - pregleduje nove in prijavljene komentarje, opozori prvožrtvovane prekrškarje, prepoved (ban) pa uvede šele po opozorilu. Sproži se ob novih komentarjih in pri prekoračitvi praga prijav (privzeti prag prijav: 3). Dovoljena orodja: `mark_comment_approved`, `mark_comment_spam`, `warn_user`, `ban_user`.

- **[Welcome Greeter](#template-welcome-greeter)** - toplo odgovori prvim komentatorjem s kratkim, osebnim pozdravom. Sproži se ob prvem komentarju novega uporabnika. Dovoljeno orodje: `write_comment`.

- **[Top Comment Pinner](#template-top-comment-pinner)** - pripne pomembne komentarje na vrhu niti, ko prekoračijo prag glasov (privzeto: 10), pri čemer najprej odneša prej pripet komentar. Sproži se ob prekoračitvi praga glasov. Dovoljena orodja: `pin_comment`, `unpin_comment`.

- **[Thread Summarizer](#template-thread-summarizer)** - objavi nevtralen, eno-odstavkovni povzetek na dolgih nitih po zamiku, nato ga pripne. Sproži se ob novih komentarjih s 30-minutnim zamikom, da se nit umiri pred povzemanjem. Dovoljena orodja: `write_comment`, `pin_comment`, `unpin_comment`.

- **[Gaslight Detector](#template-gaslight-detector)** - spremlja urejanja komentarjev za spremembe sredi niti, ki izkrivljajo odgovore, obnovi izvirno besedilo in pošlje zasebno sporočilo avtorju. Sproži se ob urejanju komentarjev. Dovoljena orodja: `edit_comment`, `warn_user`, `send_dm`.

### Prilagajanje predloge

Predloge so izhodišča, ne pogodbe. Pričakovano je, da:

- Prilagodite **Initial prompt**, da ustreza glasu vaše skupnosti.
- Dodate ali odstranite **Triggers**, da ustreza pogostosti delovanja agenta.
- Dodate **Approvals** za vsako občutljivo dejanje - močno priporočamo, da pri predlogah moderatorja zahtevate potrditev za `ban_user`.
- Dodate **Community guidelines**, da agent dosledno upošteva vašo pisno politiko. Glejte [Community Guidelines](#community-guidelines).
- Nastavite **Budgets** za posameznega agenta, primerno pričakovanemu številu sprožilcev.

Predloga je le sredstvo, ki vnaprej izpolni smiselne privzete vrednosti; ko jo shranite, je agent vaš.