An **AI Agent** je avtonomni delavec, omejen na vaš najem FastComments, ki spremlja dogodke v vaši skupnosti in v vašem imenu sprejema ukrepe.

Vsak agent ima tri stvari, ki jih nadzorujete:

1. **Osebnost.** Prosto besedilni začetni poziv, ki določa ton, vlogo in stil odločanja ("Ste toploželeni pozdravljalec skupnosti", "Uveljavljate pravila skupnosti, vendar se nagibate k opozorilom pred prepovedi", in podobno).
2. **Eden ali več sprožilcev.** Seznam dogodkov, ki prebudijo agenta – nov komentar, komentar, ki preseže prag glasov ali oznak, dejanje moderatorja, prvi komentar uporabnika na spletu in drugi. Celoten seznam je v [Trigger Events Overview](#triggers-overview).
3. **Seznam dovoljenih orodij.** Kaj je agentu dovoljeno – objaviti komentar, glasovati, pripeti, zakleniti, označiti kot spam, prepovedati uporabnika, opozoriti prek zasebnega sporočila, podeliti značko, poslati e‑pošto, shraniti in iskati v skupnem spominu. Celoten seznam je v [Allowed Tool Calls Overview](#tools-overview).

Ko se sprožilec aktivira, agent prejme sporočilo s kontekstom, ki opisuje, kaj se je zgodilo (komentar, stran, po želji kontekst niti/uporabnika/strani) a je pozvan s svojim začetnim pozivom ter vašimi smernicami skupnosti. Nato pokliče orodja za ukrepanje, pri čemer pri vsakem klicu zabeleži utemeljitev in oceno zaupanja.

### Agenti delujejo asinhrono

Agenti **nikoli ne blokirajo uporabniškega dejanja, ki jih je sprožilo**. Bralec objavi komentar, komentar se shrani in prikaže v niti, odgovor se vrne, in šele *takrat* agent deluje na njem – takoj ali po nastavitvi zamika (glejte [Deferred Triggers](#trigger-deferred-delay)). Ničesar, kar agent naredi, ne poveča zakasnitve v uporabniški izkušnji.

### Zakaj jih uporabljati

- **Moderiranje v velikem obsegu.** Označite očiten spam in prepovedujte ponavljajoče se kršitelje, ne da bi ves čas spremljali čakalno vrsto.
- **Pozdravite nove komentatorje.** Odgovorite na komentarje prvič komentirajočim v vašem glasu.
- **Izpostavite najboljšo vsebino.** Pripnite pomembne komentarje najvišje ravni, ko presegajo prag glasov.
- **Dosledno uveljavljajte smernice.** Uporabite enako besedilo politike za vsak komentar na meji.
- **Povzemite dolge niti.** Objavite nevtralne povzetke večstranskih razprav.

### Kaj vas drži pod nadzorom

- **Način suhega zagona.** Vsak nov agent je izdan v **Dry Run** načinu: obdeluje sprožilce, zažene model in zabeleži, kaj bi *storil*, vendar ne izvede dejanskih dejanj. Glejte [Dry-Run Mode](#dry-run-mode).
- **Odobritve.** Katerikoli podskupino dejanj je mogoče zakleniti za odobritev človeka. Glejte [Approval Workflow](#approval-workflow).
- **Proračuni na agenta in na račun.** Trdi dnevni in mesečni limiti. Glejte [Budgets Overview](#budgets-overview).
- **Seznam dovoljenih orodij.** Nedovoljena orodja so izrezana iz palete modela – agent jih dobesedno ne more zahtevati. Glejte [Allowed Tool Calls Overview](#tools-overview).
- **Polja revizije pri vsakem dejanju.** Model mora vključiti utemeljitev in oceno zaupanja. Obe se pojavita v časovnici zagona in pri vsaki odobritvi. Glejte [Run Detail View](#run-detail-view).
- **EU DSA člen 17.** V EU regiji so popolnoma avtomatizirane prepovedi blokirane. Glejte [EU DSA Article 17 Compliance](#eu-dsa-compliance).
- **Brez treniranja na vaših podatkih.** FastComments uporablja ponudnike, ki ne trenirajo na vaših pozivih ali komentarjih.

### Kje se ujemajo poleg človeškega moderiranja

Agenti in človeški moderatorji uporabljajo isto platformo za komentarje: agenti izvajajo dejanja prek istih kanalov (odobri, označi kot spam, prepovej, dodeli značko, pripni, zakleni, piši) in ta dejanja se pojavijo v istih [Comment Logs](/guide-moderation.html#comment-logs), na isti [Moderation Page](/guide-moderation.html#moderate-comments-page) ter v istih tokovih obvestil. Agenti in ljudje vidijo delo drug drugega in se lahko odzovejo – dejanja moderatorja so tudi veljavni sprožilci za agente (glejte [Trigger: Moderator Reviewed Comment](#trigger-moderator-reviewed) in podobno).

Če potrebujete tudi ljudi v čakalni vrsti, FastComments ponuja [Human Moderation Services](/guide-moderation.html#human-moderation-services): naši moderatorji delajo poleg vaših agentov, v vašem nadzornem plošču, po vaših smernicah.