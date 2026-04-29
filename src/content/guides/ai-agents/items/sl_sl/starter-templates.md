FastComments pošlje štiri začetne predloge, tako da vam ni treba pisati delujočega agenta iz nič. Do njih lahko dostopate na [strani AI agentov](https://fastcomments.com/auth/my-account/ai-agents) s klikom na **Brskaj po predlogah**.

Ko izberete predlogo:

1. Agent je ustvarjen z **Stanje: Preizkusno** in notranjim imenom, osnovanim na predlogi (`tos_enforcer`, `welcome_greeter`, `top_comment_pinner`, `thread_summarizer`). Če je to ime že zasedeno na vašem tenantu, se doda številčni pripona.
2. Neposredno pristanete na obrazcu za urejanje z vsem predhodno izpolnjenim - pozivom, sprožilci, dovoljenimi dejanji in morebitnimi pragovi. Na vrhu je pasica z napisom "Ustvarjeno iz predloge {templateName}. Preglejte nastavitve spodaj, nato spremenite stanje v Omogočeno, ko ste pripravljeni."
3. Še nič ni omogočeno. Agent ne bo ukrepal, dokler ne shranite in bodisi pustite preizkusno vklopljeno (za opazovanje) ali preklopite na Omogočeno.

### Štiri predloge

- **[Moderator](#template-moderator)** - pregleda nove in označene komentarje, najprej opozori prve prekrškarje, dosledično začne z izključitvijo šele po opozorilu. Sproži se ob novih komentarjih in ob presežkih praga označitev (privzeti prag označitev: 3). Dovoljena orodja: `mark_comment_approved`, `mark_comment_spam`, `warn_user`, `ban_user`.

- **[Welcome Greeter](#template-welcome-greeter)** - toplo odgovori prvim komentatorjem z kratkim, osebnim pozdravom. Sproži se ob prvem komentarju novega uporabnika. Dovoljeno orodje: `write_comment`.

- **[Top Comment Pinner](#template-top-comment-pinner)** - pripne vsebinske vrhnje komentarje, ko presežejo prag glasov (privzeto: 10), prej pripet komentar pa najprej odpri. Sproži se ob presežkih praga glasov. Dovoljena orodja: `pin_comment`, `unpin_comment`.

- **[Thread Summarizer](#template-thread-summarizer)** - objavi nevtralen enostavčen povzetek v eni odstavljku na dolgih nitih po zamudi, nato ga pripne. Sproži se ob novih komentarjih z 30-minutnim odlogom, da se razprava umiri, preden se povzame. Dovoljena orodja: `write_comment`, `pin_comment`, `unpin_comment`.

### Prilagajanje predloge

Predloge so izhodišča, ne pogodbe. Pričakuje se, da boste:

- Prilagodili **Začetni poziv**, da se ujema z glasom vaše skupnosti.
- Dodali ali odstranili **Sprožilce**, da ustreza, kako pogosto naj agent teče.
- Dodali **Odobritve** za katero koli občutljivo dejanje — močno priporočamo, da je `ban_user` zaščiten z odobritvijo za predloge v slogu moderatorja.
- Dodali **Smernice skupnosti**, da agent dosledno uporablja vašo pisno politiko. Glejte [Smernice skupnosti](#community-guidelines).
- Nastavili za posameznega agenta **Proračune**, primerne glede na to, koliko sprožilcev pričakujete.

Predloga je le vozilo, ki predizpolni smiselne privzete nastavitve; ko je shranjena, je agent vaš.