---
Sekcija **Kontekst** na obrascu za uređivanje kontrolira koliko informacija agent prima pri svakom pokretanju. Više konteksta daje bolje odluke, ali povećava trošak tokena po pokretanju, pa želite uključiti samo ono što agentu stvarno treba.

### Što je uvijek uključeno

Čak i kada su sve kućice isključene, kontekstna poruka agenta uključuje:

- Vrstu događaja okidača (npr. `COMMENT_ADD`, `COMMENT_FLAG_THRESHOLD`).
- URL stranice i ID URL-a (kad su poznati).
- Komentar koji je pokrenuo izvođenje, ako postoji - ID, ID autora, prikazno ime autora, tekst komentara, broj glasova, broj prijava, oznake spam/odobren/proveden, ID roditelja. E-mail autora **nikada** se ne šalje pružatelju LLM-a (minimizacija PII).
- Prethodni tekst komentara za okidače `COMMENT_EDIT` (tako da agent može usporediti prije/nakon).
- Smjer glasovanja za okidače `COMMENT_VOTE_THRESHOLD`.
- ID korisnika koji je pokrenuo događaj i ID značke (za okidače moderatorovih znački).

Sav nepouzdani tekst - tijela komentara, imena autora, naslovi stranica, sam dokument smjernica - **je ograničen** u kontekstnoj poruci markerima poput `<<<COMMENT_TEXT>>> ... <<<END>>>`. Sistem prompt platforme uputava model da nikad ne slijedi upute unutar tih ograda. Ovo je platformina obrana od prompt-injection napada; nije potrebno da to ponavljate u svom promptu.

### Tri kućice

#### Uključi roditeljski komentar i prethodne odgovore u istom threadu

Dodaje:
- **Roditeljski komentar** - ID, autor, tekst.
- **Sestrinski odgovori** - prethodni odgovori na istog roditelja u istom threadu.

Korisno za: bilo kojeg agenta koji odgovara na komentar u kontekstu (pozdravljatelje, sažetke threadova, moderatore koji čitaju odgovore u razgovorima).

Trošak: mali do srednji. Ograničeno brojem sestrinskih odgovora koji postoje u određenom threadu.

#### Uključi povijest komentatora: faktor povjerenja, starost računa, povijest zabrana i nedavni komentari

Dodaje blok **AUTHOR_HISTORY**:

- **Starost računa u danima** od registracije.
- **Faktor povjerenja (0-100)** - FastComments skor koji sažima koliko je korisnik pouzdan na ovom sajtu. Pogledajte stranicu [Otkrivanje spama](/guide-moderation.html#spam-detection) u vodiču za moderaciju.
- **Broj prethodnih zabrana.**
- **Ukupan broj komentara na ovom sajtu.**
- **Broj duplikata sadržaja** - ako je korisnik nedavno objavio identičan tekst (signal protiv spama).
- **Signal istog IP-a za više računa** - broj komentara s iste IP adrese pod drugim računima (signal alt-računa). Sama hash IP adrese se nikad ne šalje LLM-u.
- **Nedavni komentari** - do 5 korisnikovih najnovijih komentara, svaki skraćen na 300 znakova, ograničeni kao nepouzdani tekst.

Korisno za: bilo kojeg moderatora. Bez ovoga model zabrani nove račune i dugogodišnje korisnike dobronamjerne namjere s istim stavom.

Trošak: srednji. Nedavni komentari dodaju najviše tokena.

#### Uključi naslov stranice, podnaslov, opis i meta tagove

Dodaje blok **PAGE_CONTEXT** - naslov, podnaslov, opis i eventualne meta tagove koje je FastComments zabilježio za stranicu.

Korisno za: pozdravljatelje i sažetače threadova, gdje poznavanje o čemu se stranica radi znatno poboljšava kvalitetu izlaza.

Trošak: mali.

### Smjernice zajednice

Četvrto polje, **Smjernice zajednice**, je polje slobodnog teksta politike uključeno u kontekstnu poruku s ulogom korisnika pri svakom pokretanju, ograničeno kao nepouzdani tekst na isti način kao i tijela komentara i drugi sadržaj koji su korisnici dostavili. Agent ga čita kao tekst politike, ali platforma ga ne tretira kao sistemsku instrukciju. Pogledajte [Smjernice zajednice](#community-guidelines) za što u njega staviti.

### Selektivno dodavanje konteksta

Ove kućice primjenjuju se po agentu, a ne globalno. Uobičajeni obrasci:

- Pozdravljatelj: kontekst stranice **uključen**, kontekst thread-a **isključen**, povijest korisnika **isključena**.
- Moderator: kontekst thread-a **isključen**, povijest korisnika **uključena**, kontekst stranice **isključen**.
- Sažetač threadova: kontekst thread-a **uključen**, kontekst stranice **uključen**, povijest korisnika **isključena**.

Koristite minimalni kontekst koji agentu treba da bude ispravan za pozive koje zaista izvršava - dodatni kontekst troši tokene pri svakom pokretanju, čak i kad ga agent ne koristi.

---