---
Sekcija **Kontekst** na formi za uređivanje kontroliše koliko informacija agent prima pri svakom pokretanju. Više konteksta dovodi do boljih odluka, ali povećava cenu u tokenima po pokretanju, zato želite samo ono što je agentu zaista potrebno.

### Šta je uvek uključeno

Čak i kada nijedan čekboks nije označen, poruka konteksta agenta uključuje:

- Tip događaja koji je pokrenuo izvršavanje (npr. `COMMENT_ADD`, `COMMENT_FLAG_THRESHOLD`).
- URL stranice i ID URL-a (kad su poznati).
- Komentar koji je pokrenuo izvršavanje, ako postoji - ID, ID autora, prikazno ime autora, tekst komentara, brojevi glasova, broj prijava, spam/odobreno/pregledano zastavice, ID roditelja. Email autora se **nikada** ne šalje provajderu LLM-a (minimizacija PII).
- Prethodni tekst komentara za okidače `COMMENT_EDIT` (da bi agent mogao da uporedi pre/posle).
- Smer glasanja za okidače `COMMENT_VOTE_THRESHOLD`.
- ID korisnika koji je pokrenuo događaj i ID značke (za okidače vezane za moderator značke).
- Katalog znački vašeg tenant-a (ime, prikazani naziv, opis) kada je agentu dozvoljeno da dodeljuje značke, kako bi agent mogao da izabere odgovarajuću bez potrebe da eksplicitno nabrajate značke u promptu.

Sav nepouzdani tekst - tela komentara, imena autora, naslovi stranica, sam dokument smernica - je **ograđen** u poruci konteksta markerima kao što su `<<<COMMENT_TEXT>>> ... <<<END>>>`. Sistemski prompt platforme instruše model da nikada ne izvršava instrukcije unutar tih ograda. Ovo je platformina odbrana od prompt-injection napada; ne morate to ponavljati u svom promptu.

### Tri čekboksa

#### Include parent comment and prior replies in the same thread

Dodaje:
- **parent comment** - ID, autor, tekst.
- **Sibling replies** - prethodni odgovori na istog roditelja u istoj niti.

Korisno za: bilo koji agent koji odgovara na komentar u kontekstu (pozdravljači, sumari niti, moderatori koji čitaju odgovore u razgovorima).

Cena: mala do srednja. Ograničeno time koliko srodnih odgovora postoji u datoj niti.

#### Include commenter's trust factor, account age, ban history, and recent comments

Dodaje **AUTHOR_HISTORY** blok:

- **Starost naloga u danima** od registracije.
- **Trust factor (0-100)** - FastComments skor koji sumira koliko je korisnik pouzdan na ovom sajtu. Pogledajte stranicu [Detekcija spama](/guide-moderation.html#spam-detection) u vodiču za moderaciju.
- **Broj prethodnih zabrana.**
- **Ukupan broj komentara na ovom sajtu.**
- **Broj dupliranog sadržaja** - ako je korisnik nedavno objavio identičan tekst (signal protiv spama).
- **Signal istog IP-a preko različitih naloga** - broj komentara sa iste IP adrese pod drugim nalozima (signal alternativnih naloga). Sam hash IP-a se nikada ne šalje LLM-u.
- **Nedavni komentari** - do 5 najnovijih komentara korisnika, svaki skraćen na 300 karaktera, označeni kao nepouzdan tekst unutar ogradnih markera.

Korisno za: bilo koji moderacijski agent. Bez ovoga, model će zabranjivati nove naloge i dugoročne korisnike koji se ponašaju dobronamerno pod istim pretpostavkama.

Cena: srednja. Nedavni komentari dodaju najviše tokena.

#### Include page title, subtitle, description, and meta tags

Dodaje **PAGE_CONTEXT** blok - naslov, podnaslov, opis i svi meta tagovi koje je FastComments zabeležio za stranicu.

Korisno za: pozdravljače i sumare niti, gde poznavanje teme stranice značajno poboljšava kvalitet izlaza.

Cena: mala.

### Smernice zajednice

Četvrto polje, **Community guidelines**, je polje za slobodan tekst sa politikom koje se uključuje u poruku konteksta uloga korisnika pri svakom pokretanju, označeno kao nepouzdan tekst na isti način na koji su tela komentara i drugi korisnički sadržaji ograđeni. Agent ga čita kao tekst politike, ali platforma ga ne tretira kao sistemsku instrukciju. Pogledajte [Smernice zajednice](#community-guidelines) za šta treba staviti u njega.

### Selektivno dodavanje konteksta

Ovi čekboksovi se primenjuju po agentu, a ne globalno. Uobičajen model:

- Pozdravljač: page context **uključen**, thread context **isključen**, user history **isključen**.
- Moderator: thread context **isključen**, user history **uključen**, page context **isključen**.
- Sumator niti: thread context **uključen**, page context **uključen**, user history **isključen**.

Ciljajte na najmanji mogući kontekst koji agentu treba da bude tačan za pozive koje zaista pravi - dodatni kontekst košta tokene pri svakom pokretanju, čak i kada ga agent ne koristi.

---