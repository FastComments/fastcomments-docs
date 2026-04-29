The **Context** sekcija na formi za uređivanje kontroliše koliko informacija agent prima pri svakom pokretanju. Više konteksta daje bolje odluke, ali povećava trošak tokena po pokretanju, zato želite samo ono što agent zaista treba.

### Šta je uvijek uključeno

Čak i kada su svi čekboksovi odznačeni, agentova kontekst poruka uključuje:

- The **trigger event type** (npr. `COMMENT_ADD`, `COMMENT_FLAG_THRESHOLD`).
- The **page URL and URL ID** (kada su poznati).
- The **comment** koji je pokrenuo izvršavanje, ako postoji - ID, ID autora, prikazno ime autora, tekst komentara, brojevi glasova, broj prijava, spam/odobreno/pregledano oznake, parent ID. Email autora se **nikada** ne šalje provajderu LLM (minimizacija PII).
- The **previous comment text** za `COMMENT_EDIT` triggere (tako da agent može uporediti prije/nakon).
- The **vote direction** za `COMMENT_VOTE_THRESHOLD` triggere.
- The **triggering user ID** i **badge ID** (za triggere moderator bedževa).
- Your tenant's **badge catalog** (ime, prikazni label, opis) kada je agentu dozvoljeno nagrađivati bedževima, tako da agent može odabrati odgovarajući bez da morate nabrajati bedževe u promptu.

Sav nepouzdani tekst - tijela komentara, imena autora, naslovi stranica, sam dokument sa smjernicama - je **ograđeno** u kontekst poruci markerima kao `<<<COMMENT_TEXT>>> ... <<<END>>>`. Sistem prompt platforme naređuje modelu da nikada ne slijedi instrukcije unutar tih ograda. Ovo je platformina odbrana protiv prompt-injekcije; ne morate to ponavljati u vašem promptu.

### Tri čekboksa

#### Uključi roditeljski komentar i prethodne odgovore u istoj niti

Dodaje:
- The **parent comment** - ID, autor, tekst.
- **Sibling replies** - prethodni odgovori na istog roditelja u istoj niti.

Korisno za: svaki agent koji odgovara na komentar u kontekstu (pozdravljači, sumatori niti, moderatori koji čitaju odgovore u konverzacijama).

Trošak: mali do srednji. Ograničeno brojem srodnih odgovora u datoj niti.

#### Uključi faktor povjerenja komentatora, starost naloga, istoriju zabrana i nedavne komentare

Dodaje **AUTHOR_HISTORY** blok:

- **Starost naloga u danima** od registracije.
- **Faktor povjerenja (0-100)** - FastComments skor koji sumira koliko je korisnik pouzdan na ovom sajtu. Vidi [Spam Detection](/guide-moderation.html#spam-detection) stranicu u vodiču za moderaciju.
- **Broj prethodnih zabrana.**
- **Ukupno komentara na ovom sajtu.**
- **Broj duplikat-sadržaja** - ako je korisnik nedavno objavio identičan tekst (signal protiv spama).
- **Signal istog IP-a preko više naloga** - broj komentara sa iste IP adrese pod drugim nalozima (signal alt-naloga). Sam IP hash se nikada ne šalje LLM-u.
- **Nedavni komentari** - do 5 najnovijih komentara korisnika, svaki skraćen na 300 karaktera, ograđeni kao nepouzdani tekst.

Korisno za: bilo kojeg moderacijskog agenta. Bez ovoga, model zabranjuje nove naloge i dugotrajne korisnike u dobroj vjeri koji imaju sličan pristup.

Trošak: srednji. Nedavni komentari dodaju najviše tokena.

#### Uključi naslov stranice, podnaslov, opis i meta tagove

Dodaje **PAGE_CONTEXT** blok - naslov, podnaslov, opis i sve meta tagove koje je FastComments uhvatio za stranicu.

Korisno za: pozdravljače i sumatore niti, kada poznavanje teme stranice značajno poboljšava kvalitet izlaza.

Trošak: mali.

### Smjernice zajednice

Četvro polje, **Community guidelines**, je polje besplatnog teksta politike koje se uključuje u kontekst poruke u ulozi korisnika pri svakom pokretanju, ograđeno kao nepouzdani tekst na isti način kao i tijela komentara i drugi sadržaji koje korisnici dostave. Agent ga čita kao tekst politike, ali platforma ga ne tretira kao sistemsku instrukciju. Vidi [Smjernice zajednice](#community-guidelines) za šta treba staviti u njega.

### Selektivno dodavanje konteksta

Ovi čekboksovi se primjenjuju po agentu, a ne globalno. Uobičajeni uzorak:

- Welcome greeter: page context **on**, thread context **off**, user history **off**.
- Moderator: thread context **off**, user history **on**, page context **off**.
- Thread summarizer: thread context **on**, page context **on**, user history **off**.

Koristite minimum konteksta koji agent treba da bude tačan u pozivima koje zaista izvršava — dodatni kontekst troši tokene pri svakom pokretanju, čak i kada ga agent ne koristi.

---