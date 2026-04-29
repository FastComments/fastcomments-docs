Klikom na **Prikaži** na redu u [Istorija izvršenja](#run-history) otvara se stranica sa detaljima za pojedinačno izvršavanje. Ovde čitate rezonovanje agenta i ocenjujete njegove odluke.

### Gore: sažetak izvršavanja

- **Agent** - koji agent je pokrenut.
- **Kada** - vremenska oznaka.
- **Status** - Started / Success / Error, plus the **Probno izvršavanje** značka ako je primenljivo.
- **Trošak** - cena po izvršenju u valuti vašeg tenant-a.
- **Trošak po akciji** - trošak podeljen sa brojem ne-pending akcija, koristan za otkrivanje neuobičajeno skupih izvršenja.

### Preduzete radnje

Lista, redom, svih poziva alata koje je izvršavanje pozvalo. Svaki unos prikazuje:

- **Oznaka akcije** - "Wrote a comment", "Marked a comment as spam", "Banned a user", i slično. Oznaka se mapira iz enum tipa akcije.
- **Referentni ID** - pogođeni ID komentara, korisnika, ili značke, prikazan kao monospaced tekst (nije hyperlink).
- **Rezonovanje agenta** - obrazloženje koje je agent dostavio uz poziv.
- **Poverenje** - samoprocena poverenja agenta, prikazana kao procenat.
- **Značka na čekanju odobrenja** - ako je akcija stavljena u red u [inbox za odobrenja](#approval-workflow) umesto da je izvršena.

Ako izvršavanje nije preduzelo nijednu akciju, sekcija glasi: "Nisu preduzete nikakve akcije tokom ovog izvršavanja."

### LLM transkript

Ispod radnji, puni transkript razgovora agenta sa LLM-om:

- **Sistem** - sistemski prompt (platform suffix + vaš početni prompt + smernice zajednice).
- **Korisnik** - poruka konteksta koja opisuje okidač.
- **Asistent** - odgovori modela, uključujući pozive alata.
- **Alat** - rezultat alata vraćen modelu (npr. šta je `search_memory` vratio).

Duge poruke su sažimljive; kliknite **Proširi** / **Smanji** da biste videli.

### Čitanje transkripata

Transkript je najvažnija stranica za podešavanje. Kada agent donese odluku sa kojom se ne slažete, pročitajte ga da biste videli:

- Šta je model **video** (poruka konteksta korisnika).
- Šta je model **odlučio** (pozivi alata Asistenta).
- Šta je model **razmatrao** (bilo koji rezultati alata - npr. da li je agent zaista pozvao `search_memory` i da li je našao nešto pre zabrane).

Ako model dosledno pravi istu vrstu greške, izmenite [početni prompt](#personality-prompt) - ili upotrebite [Usavršavanje prompta](#refining-prompts) iz odbijenog odobrenja.

### Reference akcija

Referentni ID-ovi su prikazani kao monospaced tekst (nisu hyperlinkovi):

- Komentari: ID komentara.
- Korisnici: ID korisnika.
- Značke: ID značke.

Možete kopirati ID da biste pronašli pogođeni zapis na odgovarajućoj stranici za moderaciju/admin.

### Šta nedostaje u probnom izvršavanju

Probna izvršavanja prikazuju **iste** akcije, opravdanja i ocene poverenja. Jedina razlika je značka **Probno izvršavanje** na redu sa statusom. Referentni ID-ovi za komentare / korisnike / značke su i dalje prikazani - agent ih samo nije uticao.

### Greške

Za izvršavanja u `Error` stanju, stranica sa detaljima prikazuje osnovnu poruku o grešci. Uobičajene greške:

- **Nema konfigurisanog LLM API ključa** - greška u konfiguraciji tenant-a ili platforme.
- **LLM poziv je istekao** - provajder LLM-a je bio spor ili nedostupan.
- **Neuspešno slanje alatu** - agent je izabrao alat sa pogrešnim argumentima (npr. ID komentara koji više ne postoji).
- **Budžet je iskorišćen tokom izvršenja** - agentov limit je dostignut dok je izvršavanje bilo u toku. Izvršavanje je zaustavljeno.

Greške ne poništavaju delimične akcije - svi pozivi alata dovršeni pre greške ostaju.