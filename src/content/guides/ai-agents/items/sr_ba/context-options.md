The **Kontekst** sekcija u formi za uređivanje kontroliše koliko informacija agent dobija pri svakom pokretanju. Više konteksta daje bolje odluke, ali povećava trošak tokena po izvođenju, pa želite samo ono što agent zaista treba.

### Šta je uvijek uključeno

Čak i kad su sve kvačice isključene, agentova poruka konteksta uključuje:

- Tip događaja koji je izazvao okidanje (npr. `COMMENT_ADD`, `COMMENT_FLAG_THRESHOLD`).
- URL stranice i ID URL-a (kad su poznati).
- Komentar koji je izazvao pokretanje, ako postoji - ID, ID autora, prikazno ime autora, tekst komentara, brojevi glasova, broj prijava, spam/odobreno/pregledano zastavice, ID roditelja. Autorov email se **nikada** ne šalje LLM provajderu (minimalizacija PII).
- Prethodni tekst komentara za okidače `COMMENT_EDIT` (tako da agent može uporediti prije/nakon).
- Smjer glasanja za okidače `COMMENT_VOTE_THRESHOLD`.
- ID korisnika koji je izazvao okidač i ID značke (za okidače moderatorških znački).

Sav nepouzdani tekst - tijela komentara, imena autora, naslovi stranica, sam dokument smjernica - je **ogrančen** u poruci konteksta markerima poput `<<<COMMENT_TEXT>>> ... <<<END>>>`. Sistem prompt platforme naređuje modelu da nikada ne slijedi instrukcije unutar tih ograda. Ovo je platformina odbrana protiv prompt-injekcije; ne trebate to ponavljati u svom promptu.

### Tri kvačice

#### Uključi roditeljski komentar i prethodne odgovore u istoj niti

Dodaje:
- **Roditeljski komentar** - ID, autor, tekst.
- **Srodni odgovori** - prethodni odgovori na istog roditelja u istoj niti.

Koristan za: bilo koji agent koji odgovara na komentar u kontekstu (agenti za dobrodošlicu, sažimači niti, moderatori koji čitaju odgovore u razgovorima).

Trošak: mali do srednji. Ograničeno brojem odgovora koji postoje u datoj niti.

#### Uključi faktor povjerenja komentatora, starost naloga, istoriju zabrana i nedavne komentare

Dodaje **AUTHOR_HISTORY** blok:

- **Starost naloga u danima** od registracije.
- **Faktor povjerenja (0-100)** - FastComments skor koji sažima koliko je korisnik pouzdan na ovom sajtu. Pogledajte stranicu [Otkrivanje spama](/guide-moderation.html#spam-detection) u vodiču za moderaciju.
- **Broj prethodnih zabrana.**
- **Ukupan broj komentara na ovom sajtu.**
- **Broj duplog sadržaja** - ako je korisnik nedavno objavio identičan tekst (signal protiv spama).
- **Signal istog IP-a preko naloga** - broj komentara sa iste IP adrese pod drugim nalozima (signal alternativnog naloga). Sam IP hash se nikada ne šalje LLM-u.
- **Nedavni komentari** - do 5 najnovijih komentara korisnika, svaki skraćen na 300 karaktera, označeni kao nepouzdan tekst.

Koristan za: bilo koji agent za moderaciju. Bez ovoga model pogrešno zabranjuje nove naloge i dugogodišnje korisnike koji su dobronamjerni.

Trošak: srednji. Nedavni komentari dodaju najviše tokena.

#### Uključi naslov stranice, podnaslov, opis i meta tagove

Dodaje **PAGE_CONTEXT** blok - naslov, podnaslov, opis i bilo koje meta tagove koje je FastComments zabilježio za stranicu.

Koristan za: agente za dobrodošlicu i sažimače niti, gdje poznavanje o čemu se radi na stranici značajno poboljšava kvalitet izlaza.

Trošak: mali.

### Community guidelines

The fourth field, **Community guidelines**, is a free-text policy block included in the user-role context message on every run, fenced as untrusted text the same way comment bodies and other user-supplied content are fenced. The agent reads it as policy text but the platform does not treat it as a system instruction. See [Community Guidelines](#community-guidelines) for what to put in it.

### Selektivno dodavanje konteksta

Ove kvačice primjenjuju se po agentu, ne globalno. Uobičajen obrazac:

- Agent za dobrodošlicu: kontekst stranice **uključen**, kontekst niti **isključen**, istorija korisnika **isključena**.
- Moderator: kontekst niti **isključen**, istorija korisnika **uključena**, kontekst stranice **isključen**.
- Agent za sažimanje niti: kontekst niti **uključen**, kontekst stranice **uključen**, istorija korisnika **isključena**.

Koristite najmanji mogući kontekst koji agentu treba da bude tačan u pozivima koje zaista izvršava - dodatni kontekst naplaćuje tokena pri svakom pokretanju, čak i kada ga agent ne koristi.

---