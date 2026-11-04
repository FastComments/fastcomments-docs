Sekcija **Context** na formi za uređivanje kontroliše koliko informacija agent dobija pri svakom pokretanju. Više konteksta donosi bolje odluke, ali povećava trošak tokena po pokretanju, pa želite samo ono što agentu zaista treba.

### Šta je uvek uključeno

Čak i kada su svi checkbox-ovi neoznačeni, kontekstualna poruka agenta uključuje:

- **Tip događaja okidača** (npr. `COMMENT_ADD`, `COMMENT_FLAG_THRESHOLD`).
- **URL stranice i URL ID** (kada su poznati).
- **Komentar** koji je okinuo pokretanje, ako ga ima - ID, korisnički ID autora, ime autora za prikaz, tekst komentara, broj glasova, broj prijava, spam/approved/reviewed oznake, ID roditelja. Email autora se **nikada** ne šalje LLM provajderu (minimizacija PII).
- **Tekst prethodnog komentara** za `COMMENT_EDIT` okidače (da agent može da uporedi pre/posle).
- **Smer glasa** za `COMMENT_VOTE_THRESHOLD` okidače.
- **Korisnički ID okidača** i **badge ID** (za okidače moderatorskih bedževa).
- **Katalog bedževa** vašeg tenanta (naziv, oznaka za prikaz, opis) kada agent sme da dodeljuje bedževe, tako da agent može da izabere odgovarajući bez potrebe da ih nabrajate u promptu.

Sav nepoverljiv tekst - tela komentara, imena autora, naslovi stranica, sam dokument sa smernicama - je **ograđen** u kontekstualnoj poruci markerima poput `<<<COMMENT_TEXT>>> ... <<<END>>>`. Sistemski prompt platforme instruiše model da nikada ne prati instrukcije unutar tih ograda. Ovo je odbrana platforme od prompt-injection-a; ne morate je ponavljati u svom promptu.

### Tri checkbox-a

#### Uključi roditeljski komentar i prethodne odgovore u istoj niti

Dodaje:
- **Roditeljski komentar** - ID, autor, tekst.
- **Odgovore istog nivoa** - prethodne odgovore na isti roditeljski komentar u istoj niti.

Korisno za: svakog agenta koji odgovara na komentar sa kontekstom (dobrodošlice, sažimače niti, moderatore koji čitaju odgovore u razgovorima).

Trošak: mali do srednji. Ograničen brojem odgovora istog nivoa na datoj niti.

#### Uključi trust factor komentatora, starost naloga, istoriju banovanja i nedavne komentare

Dodaje blok **AUTHOR_HISTORY**:

- **Starost naloga u danima** od registracije.
- **Trust factor (0-100)** - FastComments ocena koja sumira koliko je korisnik poverljiv na ovom sajtu. Pogledajte stranicu [Spam Detection](/guide-moderation.html#spam-detection) u vodiču za moderaciju.
- **Broj prethodnih banovanja.**
- **Ukupan broj komentara na ovom sajtu.**
- **Broj dupliranih sadržaja** - ako je korisnik nedavno objavio identičan tekst (anti-spam signal).
- **Signal više naloga sa istog IP-a** - broj komentara sa istog IP-a pod drugim nalozima (signal alternativnih naloga). Sam IP hash se nikada ne šalje LLM-u.
- **Nedavni komentari** - do 5 najnovijih komentara korisnika, svaki skraćen na 300 znakova, ograđen kao nepoverljiv tekst.

Korisno za: svakog agenta za moderaciju. Bez ovoga, model banuje nove naloge i dugogodišnje korisnike dobre volje istim pristupom.

Trošak: srednji. Nedavni komentari dodaju najviše tokena.

#### Uključi naslov, podnaslov, opis stranice i meta tagove

Dodaje blok **PAGE_CONTEXT** - naslov, podnaslov, opis i sve meta tagove koje je FastComments zabeležio za stranicu.

Korisno za: dobrodošlice i sažimače niti, gde poznavanje teme stranice značajno poboljšava kvalitet izlaza.

Trošak: mali.

### Smernice zajednice

Četvrto polje, **Community guidelines**, je tekstualni blok sa pravilima koji se uključuje u kontekstualnu poruku korisničke uloge pri svakom pokretanju, ograđen kao nepoverljiv tekst na isti način na koji se ograđuju tela komentara i drugi sadržaj koji dostavljaju korisnici. Agent ga čita kao tekst pravila, ali platforma ga ne tretira kao sistemsku instrukciju. Pogledajte [Community Guidelines](#community-guidelines) za šta treba da unesete u njega.

### Selektivno dodavanje konteksta

Ovi checkbox-ovi važe po agentu, ne globalno. Čest obrazac:

- Dobrodošlica: kontekst stranice **uključen**, kontekst niti **isključen**, istorija korisnika **isključena**.
- Moderator: kontekst niti **isključen**, istorija korisnika **uključena**, kontekst stranice **isključen**.
- Sažimač niti: kontekst niti **uključen**, kontekst stranice **uključen**, istorija korisnika **isključena**.

Koristite minimalni kontekst koji agentu treba da bude tačan u pozivima koje zaista obavlja - dodatni kontekst košta tokene pri svakom pokretanju, čak i kada ga agent ne koristi.