Sekcija **Context** na obrascu za uređivanje kontrolira koliko informacija agent prima pri svakom pokretanju. Više konteksta dovodi do boljih odluka, ali povećava trošak u tokenima po pokretanju, pa želite uključiti samo ono što agentu zapravo treba.

### Što je uvijek uključeno

Čak i kada su svi potvrdni okviri isključeni, kontekstna poruka agenta uključuje:

- Vrstu **trigger event**-a (npr. `COMMENT_ADD`, `COMMENT_FLAG_THRESHOLD`).
- URL stranice i ID URL-a (kad su poznati).
- **Komentar** koji je pokrenuo izvršavanje, ako postoji - ID, ID autora, prikazno ime autora, tekst komentara, brojevi glasova, broj prijava, zastavice spam/odobreno/pregledano, parent ID. Email autora **nikada** nije poslan pružatelju LLM-a (minimalizacija PII).
- **Prethodni tekst komentara** za `COMMENT_EDIT` okidače (tako da agent može usporediti prije/nakon).
- **Smjer glasovanja** za `COMMENT_VOTE_THRESHOLD` okidače.
- ID korisnika koji je pokrenuo okidač i ID značke (za okidače vezane uz moderatorove značke).
- Katalog znački vašeg tenanta (**badge catalog**) (naziv, prikazna oznaka, opis) kada je agentu dopušteno dodjeljivati značke, kako bi agent mogao odabrati odgovarajuću bez da mu morate navoditi značke u promptu.

Sav nepouzdani tekst - tijela komentara, imena autora, naslovi stranica, sam dokument smjernica - je **zagradjen** u kontekstnoj poruci markerima poput `<<<COMMENT_TEXT>>> ... <<<END>>>`. System prompt platforme upućuje model da nikada ne izvršava upute unutar tih ograda. Ovo je platformina obrana protiv prompt-injekcija; ne trebate to ponavljati u svom promptu.

### Ta tri potvrdna okvira

#### Include parent comment and prior replies in the same thread

Dodaje:
- **Parent comment** - ID, autor, tekst.
- **Sibling replies** - prethodni odgovori na istog roditelja u istoj niti.

Korisno za: bilo koji agent koji odgovara na komentar u kontekstu (pozdravni agenti, sažimači niti, moderatori koji čitaju odgovore u konverzacijama).

Trošak: mali do srednji. Ograničeno brojem srodnih odgovora u određenoj niti.

#### Include commenter's trust factor, account age, ban history, and recent comments

Dodaje **AUTHOR_HISTORY** blok:

- **Starost računa u danima** od registracije.
- **Trust factor (0-100)** - FastComments ocjena koja sažima koliko je korisnik pouzdan na ovom mjestu. Vidi [Otkrivanje spama](/guide-moderation.html#spam-detection) stranicu u vodiču za moderaciju.
- **Broj prethodnih zabrana.**
- **Ukupan broj komentara na ovom mjestu.**
- **Broj dupliciranog sadržaja** - ako je korisnik nedavno objavio identičan tekst (signal protiv spama).
- **Signal istog IP-a preko više računa** - broj komentara s iste IP adrese pod drugim računima (signal alternativnog računa). Hash IP-a sam po sebi nikada nije poslan LLM-u.
- **Nedavni komentari** - do 5 najnovijih komentara korisnika, svaki skraćen na 300 znakova, zagradjen kao nepouzdani tekst.

Korisno za: bilo koji moderacijski agent. Bez ovih podataka model će zabranjivati nove račune i dugogodišnje korisnike dobronamjerne namjere s istim stavom.

Trošak: srednji. Nedavni komentari dodaju najviše tokena.

#### Include page title, subtitle, description, and meta tags

Dodaje **PAGE_CONTEXT** blok - naslov, podnaslov, opis i bilo koje meta oznake koje je FastComments zabilježio za stranicu.

Korisno za: pozdravne agente i sažimače niti, gdje poznavanje teme stranice značajno poboljšava kvalitetu izlaza.

Trošak: mali.

### Community guidelines

Četvrto polje, **Community guidelines**, je polje slobodnog teksta s pravilima uključeno u kontekstnu poruku uloge korisnika pri svakom pokretanju, zagradjen kao nepouzdani tekst na isti način kao tijela komentara i drugi sadržaji koje korisnici daju. Agent ga čita kao tekst politike, ali platforma ga ne tretira kao sistemsku naredbu. Pogledajte [Smjernice zajednice](#community-guidelines) za ono što treba u njega staviti.

### Dodavanje konteksta selektivno

Ovi potvrdni okviri primjenjuju se po agentu, a ne globalno. Uobičajeni obrasci:

- Pozdravni agent: page context **uključen**, thread context **isključen**, user history **isključen**.
- Moderator: thread context **isključen**, user history **uključen**, page context **isključen**.
- Sažimač niti: thread context **uključen**, page context **uključen**, user history **isključen**.

Nastojte osigurati minimalni kontekst koji agentu treba da bi bio točan u pozivima koje zapravo izvršava - dodatni kontekst košta tokene pri svakom pokretanju, čak i kada ga agent ne koristi.