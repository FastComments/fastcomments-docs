Sekcija **Context** na formi za izmenu kontroliše koliko informacija agent dobija pri svakom pokretanju. Više konteksta omogućava bolje odluke, ali povećava trošak u tokenima po pokretanju, pa želite samo ono što je agentu zaista potrebno.

### Šta je uvek uključeno

Čak i kada su sve kućice (checkbox) odčekirane, poruka konteksta agenta uključuje:

- Tip događaja koji je pokrenuo (npr. `COMMENT_ADD`, `COMMENT_FLAG_THRESHOLD`).
- URL stranice i ID URL-a (kada su poznati).
- Komentar koji je pokrenuo izvršavanje, ako postoji — ID, ID autora, prikazno ime autora, tekst komentara, brojevi glasova, broj prijava, spam/odobreno/pregledano zastavice, ID roditelja. E-mail autora se **nikada** ne šalje LLM provajderu (minimizacija PII).
- Prethodni tekst komentara za okidače `COMMENT_EDIT` (tako da agent može da uporedi pre/posle).
- Smer glasanja za okidače `COMMENT_VOTE_THRESHOLD`.
- ID korisnika koji je pokrenuo događaj i ID značke (badge) (za okidače vezane za moderator značke).

Sav nepouzdan tekst — tela komentara, imena autora, naslovi stranica, sam dokument sa smernicama — je **označen ograničivačima** u poruci konteksta markerima kao što su `<<<COMMENT_TEXT>>> ... <<<END>>>`. Sistemski prompt platforme nalaže modelu da nikada ne izvršava instrukcije unutar tih ograda. Ovo je platformina odbrana od prompt-injekcija; ne treba da ponavljate to u svom promptu.

### Tri kućice (checkbox)

#### Include parent comment and prior replies in the same thread

Dodaje:
- **Parent comment** — ID, autor, tekst.
- **Sibling replies** — prethodne odgovore na istog roditelja u istoj niti.

Koristan za: bilo koji agent koji odgovara na komentar u kontekstu (pozdravljači, sumatornici niti, moderatori koji čitaju odgovore u razgovoru).

Trošak: mali do srednji. Ograničeno brojem srodnih odgovora koji postoje u datoj niti.

#### Include commenter's trust factor, account age, ban history, and recent comments

Dodaje blok **AUTHOR_HISTORY**:

- **Starost naloga u danima** od momenta registracije.
- **Trust factor (0-100)** — FastComments skor koji sumira koliko je korisnik poverljiv na ovom sajtu. Pogledajte stranicu [Otkrivanje spama](/guide-moderation.html#spam-detection) u vodiču za moderaciju.
- **Broj prethodnih zabrana.**
- **Ukupan broj komentara na ovom sajtu.**
- **Broj dupliranog sadržaja** — ako je korisnik nedavno postavljao identičan tekst (signal protiv spama).
- **Signal deljenja IP adrese među nalozima** — broj komentara sa iste IP adrese pod drugim nalozima (signal za alternativne naloge). Sam hash IP adrese se nikada ne šalje LLM-u.
- **Najnoviji komentari** — do 5 najnovijih komentara korisnika, svaki skraćen na 300 karaktera i označen markerima kao nepouzdan tekst.

Koristan za: bilo kog moderatora. Bez ovoga, model pogrešno zabranjuje nove naloge i dugogodišnje korisnike koji deluju dobronamerno.

Trošak: srednji. Najnoviji komentari dodaju najviše tokena.

#### Include page title, subtitle, description, and meta tags

Dodaje blok **PAGE_CONTEXT** — naslov, podnaslov, opis i bilo koje meta tagove koje je FastComments prikupio za stranicu.

Koristan za: pozdravljače i sumatornike niti, gde poznavanje o čemu se stranica radi značajno poboljšava kvalitet izlaza.

Trošak: mali.

### Smernice zajednice

Četvrvo polje, **Community guidelines**, je polje sa slobodnim tekstom politike koje se uključuje u poruku konteksta sa korisničkom ulogom pri svakom pokretanju, označeno kao nepouzdan tekst na isti način kao i tela komentara i drugi sadržaji koje korisnici dostavljaju. Agent ga čita kao tekst politike, ali platforma ga ne tretira kao sistemsku instrukciju. Pogledajte [Community Guidelines](#community-guidelines) za ono što treba staviti u njega.

### Selektivno dodavanje konteksta

Ove kućice (checkbox) se primenjuju po agentu, ne globalno. Čest obrazac:

- Agent za pozdravljanje: page context **on**, thread context **off**, user history **off**.
- Moderator: thread context **off**, user history **on**, page context **off**.
- Agent za sažimanje niti: thread context **on**, page context **on**, user history **off**.

Ciljajte na minimalni kontekst koji agentu treba da bi tačno obavio pozive koje zaista pravi — dodatni kontekst naplaćuje tokene pri svakom pokretanju, čak i kada ga agent ne koristi.