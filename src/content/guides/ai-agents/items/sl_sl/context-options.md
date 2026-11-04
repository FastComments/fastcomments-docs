Razdelek **Kontekst** na obrazcu za urejanje nadzoruje, koliko informacij agent prejme ob vsakem zagonu. Več konteksta prinese boljše odločitve, a poveča stroške žetonov na zagon, zato želite le tisto, kar agent dejansko potrebuje.

### Kaj je vedno vključeno

Tudi če so vsa potrditvena polja odkljukana, sporočilo konteksta agenta vključuje:

- **Vrsto sprožilnega dogodka** (npr. `COMMENT_ADD`, `COMMENT_FLAG_THRESHOLD`).
- **URL strani in URL ID** (če sta znana).
- **Komentar**, ki je sprožil zagon, če obstaja - ID, ID uporabnika avtorja, prikazano ime avtorja, besedilo komentarja, število glasov, število prijav, oznake spam/odobreno/pregledano, ID starša. E-poštni naslov avtorja **nikoli** ni poslan ponudniku LLM (minimizacija PII).
- **Prejšnje besedilo komentarja** za sprožilce `COMMENT_EDIT` (da agent lahko primerja stanje pred/po).
- **Smer glasovanja** za sprožilce `COMMENT_VOTE_THRESHOLD`.
- **ID sprožilnega uporabnika** in **ID značke** (za sprožilce moderatorskih značk).
- **Katalog značk** vašega najemnika (ime, prikazana oznaka, opis), kadar agent sme podeljevati značke, tako da agent lahko izbere ustrezno, ne da bi jih vam bilo treba našteti v pozivu.

Vse nezaupljivo besedilo - telesa komentarjev, imena avtorjev, naslovi strani, sam dokument smernic - je v sporočilu konteksta **ograjeno** z označevalci, kot je `<<<COMMENT_TEXT>>> ... <<<END>>>`. Sistemski poziv platforme modelu naroča, da nikoli ne sledi navodilom znotraj teh ograj. To je obramba platforme pred vbrizgavanjem pozivov; v svojem pozivu je ni treba ponavljati.

### Trije potrditveni gumbi

#### Vključi starševski komentar in prejšnje odgovore v isti niti

Doda:
- **Starševski komentar** - ID, avtor, besedilo.
- **Sorojenčne odgovore** - prejšnje odgovore na istega starša v isti niti.

Uporabno za: vsakega agenta, ki odgovarja na komentar v kontekstu (pozdravljalci, povzemalci niti, moderatorji, ki berejo odgovore v pogovorih).

Strošek: majhen do srednji. Omejen s številom sorojencev na dani niti.

#### Vključi faktor zaupanja komentatorja, starost računa, zgodovino prepovedi in nedavne komentarje

Doda blok **AUTHOR_HISTORY**:

- **Starost računa v dnevih** od registracije.
- **Faktor zaupanja (0-100)** - FastComments ocena, ki povzema, kako zaupanja vreden je uporabnik na tem spletnem mestu. Glejte stran [Zaznavanje neželene pošte](/guide-moderation.html#spam-detection) v priročniku za moderiranje.
- **Število prejšnjih prepovedi.**
- **Skupno število komentarjev na tem spletnem mestu.**
- **Število podvojenih vsebin** - če je uporabnik nedavno objavil identično besedilo (signal proti neželeni pošti).
- **Signal istega IP-ja med računi** - število komentarjev z istega IP-ja pod drugimi računi (signal alternativnega računa). Samo zgoščena vrednost IP-ja nikoli ni poslana LLM.
- **Nedavni komentarji** - do 5 najnovejših komentarjev uporabnika, vsak skrajšan na 300 znakov, ograjen kot nezaupljivo besedilo.

Uporabno za: vsakega moderatorskega agenta. Brez tega model prepoveduje nove račune in dolgoletne uporabnike z dobrimi namenami z enako strogostjo.

Strošek: srednji. Nedavni komentarji dodajo največ žetonov.

#### Vključi naslov strani, podnaslov, opis in meta oznake

Doda blok **PAGE_CONTEXT** - naslov, podnaslov, opis in vse meta oznake, ki jih je FastComments zajel za stran.

Uporabno za: pozdravljalce in povzemalce niti, kjer poznavanje teme strani znatno izboljša kakovost izhoda.

Strošek: majhen.

### Smernice skupnosti

Četrto polje, **Smernice skupnosti**, je poljubno besedilni blok pravil, ki je vključen v sporočilo konteksta uporabniške vloge ob vsakem zagonu, ograjen kot nezaupljivo besedilo na enak način kot telesa komentarjev in druga uporabniška vsebina. Agent ga bere kot besedilo pravil, platforma pa ga ne obravnava kot sistemsko navodilo. Glejte [Smernice skupnosti](#community-guidelines) za vsebino, ki jo vanj vnesti.

### Izbirno dodajanje konteksta

Ta potrditvena polja veljajo za posameznega agenta, ne globalno. Pogost vzorec:

- Pozdravljalec: kontekst strani **vklopljen**, kontekst niti **izklopljen**, zgodovina uporabnika **izklopljena**.
- Moderator: kontekst niti **izklopljen**, zgodovina uporabnika **vklopljena**, kontekst strani **izklopljen**.
- Povzemalec niti: kontekst niti **vklopljen**, kontekst strani **vklopljen**, zgodovina uporabnika **izklopljena**.

Uporabite najmanj konteksta, ki ga agent potrebuje, da je pravilen pri klicih, ki jih dejansko opravlja - odvečni kontekst stane žetone ob vsakem zagonu, tudi kadar ga agent ne uporabi.