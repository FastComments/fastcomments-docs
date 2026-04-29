Razdelek **Kontekst** na obrazcu za urejanje nadzoruje, koliko informacij agent prejme pri vsakem izvajanju. Več konteksta pripelje do boljših odločitev, vendar poveča strošek tokenov na zagon, zato želite imeti le tisto, kar agent dejansko potrebuje.

### Kaj je vedno vključeno

Tudi če niso označena nobena potrditvena polja, sporočilo s kontekstom agenta vključuje:

- Vrsto sprožilnega dogodka (npr. `COMMENT_ADD`, `COMMENT_FLAG_THRESHOLD`).
- URL strani in ID URL (če sta znana).
- Komentar, ki je sprožil zagon, če obstaja - ID, ID avtorja, prikazno ime avtorja, besedilo komentarja, število glasov, število označitev, zastavice spam/approved/reviewed, ID nadrejenega. E-pošta avtorja se ponudniku LLM nikoli ne pošlje (minimizacija osebnih podatkov).
- Prejšnje besedilo komentarja za sprožilce `COMMENT_EDIT` (da lahko agent primerja pred/po).
- Smer glasovanja za sprožilce `COMMENT_VOTE_THRESHOLD`.
- ID uporabnika, ki je sprožil dogodek, in ID značke (za sprožilce značk moderatorjev).

Vsa nezaupanja besedila - telesa komentarjev, imena avtorjev, naslovi strani, sam dokument smernic - so v sporočilu s kontekstom OGRADENA z markerji, kot so `<<<COMMENT_TEXT>>> ... <<<END>>>`. Sistemsko navodilo platforme modelu zapoveduje, naj nikoli ne sledi navodilom znotraj teh ograj. To je obramba platforme pred vbrizgavanjem pozivov; vam ni treba tega ponavljati v svojem pozivu.

### Tri potrditvena polja

#### Vključi nadrejeni komentar in prejšnje odgovore v isti niti

Doda:
- Nadrejeni komentar - ID, avtor, besedilo.
- Sorodni odgovori - prejšnji odgovori na istega nadrejenega v isti niti.

Uporabno za: katerega koli agenta, ki odgovarja na komentar v kontekstu (agent za pozdravljanje, povzemač niti, moderatorji, ki berejo odgovore v pogovorih).

Strošek: majhen do srednji. Omejeno s številom sorodnih odgovorov, ki obstajajo v določeni niti.

#### Vključi faktor zaupanja komentarista, starost računa, zgodovino prepovedi in nedavne komentarje

Doda blok AUTHOR_HISTORY:

- Starost računa v dnevih od registracije.
- Faktor zaupanja (0-100) - FastComments ocena, ki povzema, kako zaupanja vreden je uporabnik na tej strani. Glejte [Odkrivanje neželene pošte](/guide-moderation.html#spam-detection) v vodiču za moderiranje.
- Število prejšnjih prepovedi.
- Skupno število komentarjev na tej strani.
- Število podvojenih vsebin - če je uporabnik nedavno objavil enako besedilo (signal proti neželeni pošti).
- Signal o več računih z istega IP - štetje komentarjev z istega IP pod drugimi računi (signal za alternativne račune). Zgoščena vrednost IP sama nikoli ni poslana LLM.
- Nedavni komentarji - do 5 najnovejših komentarjev uporabnika, vsak skrajšan na 300 znakov, ograjen kot nezaupanja vredno besedilo.

Uporabno za: katerega koli moderacijskega agenta. Brez tega model pogosto prepove nove račune in dolgoletne, dobroverne uporabnike z enakim vzorcem vedenja.

Strošek: srednji. Nedavni komentarji dodajo največ tokenov.

#### Vključi naslov strani, podnaslov, opis in meta oznake

Doda blok PAGE_CONTEXT - naslov, podnaslov, opis in vse meta oznake, ki jih je FastComments zajel za stran.

Uporabno za: agente za pozdravljanje in povzemače niti, kjer znanje, o čem govori stran, bistveno izboljša kakovost izhoda.

Strošek: majhen.

### Smernice skupnosti

Četrto polje, **Smernice skupnosti**, je polje z besedilom police, vključeno v sporočilo s kontekstom v vlogi uporabnika pri vsakem izvajanju, ogranjeno kot nezaupanja vredno besedilo na enak način kot telesa komentarjev in druge vsebine, ki jih vnese uporabnik. Agent jih bere kot besedilo z navodili, vendar platforma tega ne obravnava kot sistemsko navodilo. Glejte [Smernice skupnosti](#community-guidelines) za navodila, kaj naj vključite vanj.

### Selektivno dodajanje konteksta

Ta potrditvena polja veljajo za posameznega agenta, ne globalno. Pogost vzorec:

- Agent za pozdravljanje: kontekst strani vključen, kontekst niti izključen, zgodovina uporabnika izključena.
- Moderator: kontekst niti izključen, zgodovina uporabnika vključena, kontekst strani izključen.
- Povzemalec niti: kontekst niti vključen, kontekst strani vključen, zgodovina uporabnika izključena.

Uporabljajte najmanjši kontekst, ki ga agent potrebuje, da pravilno opravi klice, ki jih dejansko izvede - dodatni kontekst stane tokenov pri vsakem zagonu, tudi ko ga agent ne uporablja.