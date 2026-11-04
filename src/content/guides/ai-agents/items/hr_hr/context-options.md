Sekcija **Context** na obrascu za uređivanje kontrolira koliko informacija agent prima pri svakom pokretanju. Više konteksta donosi bolje odluke, ali povećava trošak tokena po pokretanju, pa želite samo ono što agent stvarno treba.

### Što je uvijek uključeno

Čak i ako su svi potvrdni okviri poništeni, poruka konteksta agenta uključuje:

- **Vrstu događaja okidača** (npr. `COMMENT_ADD`, `COMMENT_FLAG_THRESHOLD`).
- **URL stranice i URL ID** (kada su poznati).
- **Komentar** koji je pokrenuo pokretanje, ako postoji - ID, korisnički ID autora, ime autora za prikaz, tekst komentara, broj glasova, broj prijava, oznake spam/odobreno/provjereno, ID roditelja. Autorova e-pošta se **nikada** ne šalje LLM pružatelju usluga (minimizacija PII podataka).
- **Prethodni tekst komentara** za okidače `COMMENT_EDIT` (tako agent može usporediti stanje prije/poslije).
- **Smjer glasanja** za okidače `COMMENT_VOTE_THRESHOLD`.
- **Korisnički ID koji je okinuo događaj** i **ID značke** (za okidače moderatorskih znački).
- **Katalog znački** vašeg najmoprimca (naziv, oznaka za prikaz, opis) kada agent smije dodjeljivati značke, tako agent može odabrati odgovarajuću bez da ih vi morate opisati u upitu.

Sav nepouzdani tekst - tijela komentara, imena autora, naslovi stranica, sam dokument smjernica - je **ograđen** u poruci konteksta markerima poput `<<<COMMENT_TEXT>>> ... <<<END>>>`. Sistemski upit platforme upućuje model da nikada ne slijedi upute unutar tih ograda. To je obrana platforme od ubacivanja upita; ne trebate je ponavljati u svom upitu.

### Tri potvrdna okvira

#### Uključi roditeljski komentar i prethodne odgovore u istoj niti

Dodaje:
- **Roditeljski komentar** - ID, autor, tekst.
- **Odgovore braće/sestara** - prethodne odgovore na isti roditeljski komentar u istoj niti.

Korisno za: svakog agenta koji odgovara na komentar u kontekstu (dobrodošlice, sažimače niti, moderatore koji čitaju odgovore u razgovorima).

Trošak: mali do srednji. Ograničeno brojem odgovora koji postoje na danoj niti.

#### Uključi faktor povjerenja komentatora, starost računa, povijest banova i nedavne komentare

Dodaje blok **AUTHOR_HISTORY**:

- **Starost računa u danima** od registracije.
- **Faktor povjerenja (0-100)** - FastComments rezultat koji sažima koliko je korisnik pouzdan na ovom sajtu. Pogledajte stranicu [Otkrivanje spama](/guide-moderation.html#spam-detection) u vodiču za moderaciju.
- **Broj prethodnih banova.**
- **Ukupno komentara na ovom sajtu.**
- **Broj dupliciranih sadržaja** - ako je korisnik nedavno objavio identičan tekst (signal protiv spama).
- **Signal istog IP-a preko više računa** - broj komentara s istog IP-a pod drugim računima (signal alternativnog računa). Sam IP hash se nikada ne šalje LLM-u.
- **Nedavni komentari** - do 5 korisnikovih najnovijih komentara, svaki skraćen na 300 znakova, ograđeno kao nepouzdani tekst.

Korisno za: svakog agenta za moderaciju. Bez toga, model banuje nove račune i dugogodišnje korisnike dobre vjeri istim pristupom.

Trošak: srednji. Nedavni komentari dodaju najviše tokena.

#### Uključi naslov stranice, podnaslov, opis i meta oznake

Dodaje blok **PAGE_CONTEXT** - naslov, podnaslov, opis i sve meta oznake koje je FastComments zabilježio za stranicu.

Korisno za: dobrodošlice i sažimače niti, gdje poznavanje teme stranice znatno poboljšava kvalitetu rezultata.

Trošak: mali.

### Smjernice zajednice

Četvrto polje, **Community guidelines**, je tekstualni blok pravila slobodnog formata koji se uključuje u poruku konteksta korisničke uloge pri svakom pokretanju, ograđen kao nepouzdani tekst na isti način na koji se ograđuju tijela komentara i drugi korisnički sadržaj. Agent ga čita kao tekst pravila, ali ga platforma ne tretira kao sistemsku instrukciju. Pogledajte [Smjernice zajednice](#community-guidelines) za što upisati u njega.

### Selektivno dodavanje konteksta

Ovi potvrdni okviri primjenjuju se po agentu, ne globalno. Čest obrazac:

- Dobrodošlica: kontekst stranice **uključen**, kontekst niti **isključen**, povijest korisnika **isključena**.
- Moderator: kontekst niti **isključen**, povijest korisnika **uključena**, kontekst stranice **isključen**.
- Sažimač niti: kontekst niti **uključen**, kontekst stranice **uključen**, povijest korisnika **isključena**.

Koristite minimalni kontekst koji agent treba da bi bio točan u pozivima koje stvarno upućuje - dodatni kontekst košta tokene pri svakom pokretanju, čak i kada ga agent ne koristi.