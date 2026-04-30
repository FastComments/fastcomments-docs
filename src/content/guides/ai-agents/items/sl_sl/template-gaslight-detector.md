**ID predloge:** `gaslight_detector`

Detektor gaslighta opazuje urejanja komentarjev, ki prepišejo zgodovino sredi pogovora - takšne, kjer avtor spremeni pomen prej napisanega komentarja po tem, ko so bile nanj napisane odgovore, zaradi česar se kasnejši odgovori zdijo iz konteksta ali napačni. Ko agent oceni, da urejanje preseže to mejo, obnovi izvirno besedilo in avtorju pošlje zasebno sporočilo z razlago.

To je predloga z višjim tveganjem, ker spreminja vsebino uporabnika. Zaženite jo v [suhem zagonu](#dry-run-mode) dlje, kot bi to storili za predlogo samo za branje, in postavite `edit_comment` za [odobritev](#approval-workflow), dokler ne zaupate presoji modela glede vašega prometa.

### Sprožilci

- **Urejanje komentarja** (`COMMENT_EDIT`) - agent primerja novo in prejšnjo vsebino ter odloči, ali urejanje izkrivlja odgovore, ki že obstajajo.

Oglejte si [Sprožilec: Urejanje komentarja](#trigger-comment-edit) za celoten nabor podatkov (payload), vključno s prejšnjim besedilom komentarja in številom odgovorov ob času urejanja.

### Dovoljena orodja

- [`edit_comment`](#tool-edit-comment) - uporablja se za obnovitev izvirnega besedila, kadar je urejanje ocenjeno kot gaslighting.
- [`warn_user`](#tool-warn-user) - izda mehko opozorilo, ki ga uporabnik vidi ob naslednjem obisku.
- [`send_dm`](#tools-overview) - kanal za razlago; uporabnik prejme zasebno sporočilo, v katerem je opisano, zakaj je bilo njegovo urejanje razveljavljeno.

Ne more prepovedovati, označiti kot neželeno pošto, glasovati ali objavljati novih komentarjev - obseg je namenoma ozek.

### Priporočene dodatne nastavitve pred začetkom delovanja

- **Postavite `edit_comment` za [odobritev](#approval-workflow).** Razveljavitev komentarja je vidna avtorju in vsem, ki so videli urejeno različico, zato je lažni pozitiven rezultat lahko neroden. Ohranite odobritve vklopljene, dokler suhi zagon ne pokaže, da je agent dosleden.
- **Zožite poziv (prompt) s primeri, kaj na vašem mestu šteje kot gaslighting.** Privzeti poziv je namenoma kratek. Modelu podajte konkretne primere - "obrat trditve da/ne", "brisanje številke, na katero se odgovori sklicujejo", "dodan sovražen stavek po tem, ko so bili objavljeni odgovori" - in tudi izrecne ne-primere, kot so popravki tipk, čiščenje oblikovanja ali dodajanje virov.
- **Uporabite število odgovorov iz konteksta sprožilca.** Urejanja komentarjev z nič odgovori ne morejo izkriviti pogovora; poziv naj modelu pove, naj ta izpušči takšna urejanja.
- **Označite "Vključi dejavnik zaupanja komentatorja, starost računa, zgodovino prepovedi in nedavne komentarje"** v [Možnosti konteksta](#context-options). Model je veliko manj agresiven, kadar vidi dolgoletni račun, ki deluje v dobri veri.
- **Razmislite o kratkem obdobju milosti za urejanje v pozivu.** Veliko urejanj v prvih 30–60 sekundah so popravki tipk; navodite model, naj prezre tako hitra urejanja.

### Priporočeno obdobje suhega zagona

Zaženite vsaj dva tedna z dejanskim prometom v [suhem zagonu](#dry-run-mode) pred preklopom na Omogočeno (Enabled) in med tem obdobjem preglejte vsako označeno urejanje. Uporabite [Preskusi (ponovitve)](#test-runs-replays) za predvajanje zadnjih 30 dni urejanj proti agentu pred začetkom delovanja.