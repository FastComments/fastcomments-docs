Sekcija **Context** na obrascu za uređivanje kontroliše koliko informacija agent dobija pri svakom izvršavanju. Više konteksta dovodi do boljih odluka, ali povećava trošak tokena po izvršavanju, pa želite samo ono što agentu zaista treba.

### Šta je uvek uključeno

Čak i kada su svi okviri za potvrdu isključeni, poruka konteksta agenta uključuje:

- Tip događaja koji pokreće (`COMMENT_ADD`, `COMMENT_FLAG_THRESHOLD`).
- URL stranice i URL ID (kad su poznati).
- Komentar koji je pokrenuo izvršavanje, ako ga ima - ID, ID autora, prikazno ime autora, tekst komentara, brojevi glasova, broj prijava, zastavice spam/odobreno/pregledano, parent ID. Email autora se **nikada** ne šalje provajderu LLM (minimizacija PII).
- Prethodni tekst komentara za `COMMENT_EDIT` okidače (tako da agent može da uporedi pre/posle).
- Smer glasanja za `COMMENT_VOTE_THRESHOLD` okidače.
- ID korisnika koji je pokrenuo događaj i ID značke (za okidače koje koriste moderator značke).

Sav nepouzdani tekst - tela komentara, imena autora, naslovi stranica, sam dokument smernica - je **označen ograničivačima** u poruci konteksta markerima poput `<<<COMMENT_TEXT>>> ... <<<END>>>`. Sistemski prompt platforme upućuje model da nikada ne izvršava instrukcije unutar tih ogradnih markera. Ovo je odbrana platforme protiv prompt-injection napada; ne morate to da ponavljate u svom promptu.

### Tri polja za potvrdu

#### Include parent comment and prior replies in the same thread

Dodaje:
- **Parent comment** - ID, autor, tekst.
- **Sibling replies** - prethodni odgovori na istog roditelja u istoj niti.

Korisno za: bilo kog agenta koji odgovara na komentar u kontekstu (pozdravljače, sažimače niti, moderatore koji čitaju odgovore u razgovorima).

Trošak: mali do srednji. Ograničen brojem odgovora na istom nivou u datoj niti.

#### Include commenter's trust factor, account age, ban history, and recent comments

Dodaje **AUTHOR_HISTORY** blok:

- **Account age in days** od registracije.
- **Trust factor (0-100)** - FastComments skor koji sažima koliko je korisnik pouzdan na ovom sajtu. Pogledajte [Otkrivanje spama](/guide-moderation.html#spam-detection) stranicu u vodiču za moderaciju.
- **Prethodni broj banova.**
- **Ukupan broj komentara na ovom sajtu.**
- **Broj duplog sadržaja** - ako je korisnik nedavno postavio identičan tekst (signal protiv spama).
- **Signal istog IP-a za više naloga** - broj komentara sa iste IP adrese pod drugim nalozima (signal alternativnog naloga). Sam heš IP adrese se nikada ne šalje LLM-u.
- **Nedavni komentari** - do 5 najnovijih komentara korisnika, svaki skraćen na 300 karaktera, obeleženi ograničivačima kao nepouzdani tekst.

Korisno za: bilo kog moderator agenta. Bez ovoga model greškom briše nove naloge i dugoročne korisnike dobronamernog ponašanja koji imaju sličan obrazac.

Trošak: srednji. Nedavni komentari dodaju najviše tokena.

#### Include page title, subtitle, description, and meta tags

Dodaje **PAGE_CONTEXT** blok - naslov, podnaslov, opis i bilo koje meta tagove koje je FastComments zabeležio za stranicu.

Korisno za: pozdravljače i sažimače niti, gde poznavanje o čemu je stranica značajno poboljšava kvalitet izlaza.

Trošak: mali.

### Smernice zajednice

Četvrto polje, **Community guidelines**, je polje slobodnog teksta sa politikom koje se uključuje u poruku konteksta u ulozi korisnika pri svakom izvršavanju, obeleženo ograničivačima kao nepouzdani tekst na isti način kao tela komentara i drugi sadržaj koji korisnik obezbeđuje. Agent ga čita kao tekst politike, ali platforma ga ne tretira kao sistemsku instrukciju. Pogledajte [Smernice zajednice](#community-guidelines) za šta treba da stavite u njega.

### Dodavanje konteksta selektivno

Ova polja za potvrdu važe po agentu, a ne globalno. Uobičajen obrazac:

- Welcome greeter: kontekst stranice UKLJUČEN, kontekst niti ISKLJUČEN, istorija korisnika ISKLJUČENA.
- Moderator: kontekst niti ISKLJUČEN, istorija korisnika UKLJUČENA, kontekst stranice ISKLJUČEN.
- Sažimač niti: kontekst niti UKLJUČEN, kontekst stranice UKLJUČEN, istorija korisnika ISKLJUČENA.

Ciljajte na minimum konteksta koji agentu treba da bude ispravan za pozive koje zaista obavlja - dodatni kontekst povećava potrošnju tokena pri svakom izvršavanju, čak i kada ga agent ne koristi.