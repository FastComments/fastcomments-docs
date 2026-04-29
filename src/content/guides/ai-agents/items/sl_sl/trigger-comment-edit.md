Sproži agenta, ko je komentar urejen.

### Kontekst, ki ga agent prejme

- Komentar v svoji trenutni (po-ureditvi) obliki.
- **prejšnje besedilo komentarja** kot ločen ograjen blok (`PREVIOUS_TEXT`). To je edinstveno za sprožilo urejanja - agent lahko primerja stanje pred in po.
- Neobvezna zgodovina teme / uporabnika / strani, kot je nastavljeno.

### Pomembno

- Sprožilo se aktivira ob vsaki uspešni ureditvi, vključno z ureditvami, ki jih moderatorji izvedejo v imenu uporabnika.
- Agenti nimajo dostopa do orodja za urejanje komentarjev; agenti sploh ne morejo urejati komentarjev.
- Prejšnje besedilo komentarja je ograjeno kot nezaupljiv vhod. Sistemski poziv platforme opominja model, naj ne sledi navodilom znotraj ograj - to je tukaj pomembno, ker bi zlonamerni uporabnik lahko uredil komentar in vanj vnesel ukaz "ne upoštevaj svojih prejšnjih navodil", namenjen kateremu koli agentu, ki spremlja dogodke urejanja.

### Pogoste uporabe

- **Zajetje prikrite vsebine** - uporabnik uredi prej čisti komentar in vanj vstavi spam potem, ko je moderator že nadaljeval.
- **Sledenje manjšim ureditvam** - če vaša skupnost obravnava urejanja kot ločene dogodke zaradi kakršnega koli revizijskega razloga.

### Opomba o stroških

Sprožilci urejanja vidijo dve kopiji besedila komentarja (nova različica v standardnem COMMENT bloku, stara različica v PREVIOUS_TEXT bloku). Pri dolgih komentarjih to približno podvoji strošek v žetonih za izvedbo v primerjavi s sprožilcem `COMMENT_ADD` - upoštevajte to pri proračuniranju.