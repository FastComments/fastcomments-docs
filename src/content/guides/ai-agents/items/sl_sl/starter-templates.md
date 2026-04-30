FastComments vključuje pet izhodiščnih predlog, tako da vam ni treba pisati delujočega agenta iz nič. Do njih pridete na strani [AI Agents page](https://fastcomments.com/auth/my-account/ai-agents) tako, da kliknete **Browse templates**.

Ko izberete predlogo:

1. Agent je ustvarjen z **Status: Dry Run** in notranjim imenom, ki temelji na predlogi (`tos_enforcer`, `welcome_greeter`, `top_comment_pinner`, `thread_summarizer`, `gaslight_detector`). Če je to ime že zasedeno v vašem najemniku, se doda številčni pripona.
2. Pristanete neposredno na obrazcu za urejanje z vsem vnaprej izpolnjenim - pozivom (prompt), sprožilci, dovoljenimi dejanji in morebitnimi pragovi. Na vrhu se prikaže pasica z besedilom "Created from the {templateName} template. Review the settings below, then change status to Enabled when you're ready."
3. Nič še ni omogočeno. Agent ne bo deloval, dokler ne shranite in bodisi obdržite dry-run vklopljen (za opazovanje), ali preklopite na Omogočeno.

### The five templates

- **[Moderator](#template-moderator)** - pregleda nove in prijavljene komentarje, opozori kršitelje ob prvem primeru in prične z blokado šele po opozorilu. Sproži se ob novih komentarjih in pri presežkih praga prijav (privzeti prag prijav: 3). Dovoljena orodja: `mark_comment_approved`, `mark_comment_spam`, `warn_user`, `ban_user`.

- **[Welcome Greeter](#template-welcome-greeter)** - toplo odgovori prvim komentatorjem z kratkim, osebnim pozdravom. Sproži se ob first-time komentarju novega uporabnika. Dovoljeno orodje: `write_comment`.

- **[Top Comment Pinner](#template-top-comment-pinner)** - pripne vsebinske komentarje na najvišji ravni, ko presežejo prag glasov (privzeto: 10), pri tem najprej odlepi prej pripet komentar. Sproži se ob presežkih praga glasov. Dovoljena orodja: `pin_comment`, `unpin_comment`.

- **[Thread Summarizer](#template-thread-summarizer)** - objavi nevtralen, enostavčen povzetek v enem odstavku na dolgih nitih po zamiku in ga nato pripne. Sproži se ob novih komentarjih z 30-minutnim odlogom, da se nit umiri pred povzemanjem. Dovoljena orodja: `write_comment`, `pin_comment`, `unpin_comment`.

- **[Gaslight Detector](#template-gaslight-detector)** - spremlja urejanja komentarjev zaradi prepisov v sredini niti, ki izkrivljajo odgovore, obnovi prvotno besedilo in avtorju pošlje zasebno sporočilo. Sproži se ob urejanju komentarjev. Dovoljena orodja: `edit_comment`, `warn_user`, `send_dm`.

### Customizing a template

Predloge so izhodišča, ne pogodbe. Pričakuje se, da boste:

- Prilagodili **Initial prompt**, da bo ustrezal glasu vaše skupnosti.
- Dodali ali odstranili **Triggers**, da ustreza, kako pogosto naj agent teče.
- Dodali **Approvals** za vsako občutljivo dejanje - močno priporočamo, da `ban_user` za predloge v slogu moderatorja zahtevate soglasje.
- Dodali **Community guidelines**, da agent dosledno uporablja vašo pisno politiko. Oglejte si [Community Guidelines](#community-guidelines).
- Nastavili za vsakega agenta **Budgets**, primerne glede na to, koliko sprožilcev pričakujete.

Predloga je le vozilo, ki vnaprej izpolni smiselne privzete vrednosti; ko jo shranite, je agent vaš.