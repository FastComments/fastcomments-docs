Klikom na **Prikaži** na redu u [Istorija izvršavanja](#run-history) otvara se stranica sa detaljima za pojedinačno izvršavanje. Ovde čitate rezonovanje agenta i ocenjujete njegove odluke.

### Gore: rezime izvršavanja

- **Agent** - koji agent je pokrenut.
- **Kada** - vremenska oznaka.
- **Status** - Started / Success / Error, plus the **Dry Run** bedž ako je primenljivo.
- **Cost** - trošak po izvršavanju u valuti vašeg tenanta.
- **Cost per action** - trošak podeljen sa brojem ne-pending akcija, korisno za uočavanje neuobičajeno skupih izvršavanja.

### Preduzete akcije

Lista, redom, svih poziva alata koje je izvršavanje napravilo. Svaki unos prikazuje:

- **Action label** - "Napisao komentar", "Označio komentar kao spam", "Zabranio korisnika", i tako dalje. Oznaka potiče iz action type enum-a.
- **Reference ID** - ID pogođenog komentara, korisnika ili bedža, prikazan kao monospace tekst (nije hyperlink).
- **Agent reasoning** - obrazloženje koje je agent priložio uz poziv.
- **Confidence** - agentova sopstvena ocena poverenja, prikazana kao procenat.
- **Pending approval** bedž - ako je akcija u redu za [pretinac za odobrenja](#approval-workflow) umesto da je izvršena.

Ako izvršavanje nije preduzelo nijednu akciju, sekcija prikazuje: "Tokom ovog izvršavanja nisu preduzete nikakve akcije."

### Transkript LLM-a

Ispod akcija nalazi se kompletan transkript razgovora agenta sa LLM-om:

- **System** - sistemski prompt (platform suffix + vaš početni prompt + smernice zajednice).
- **User** - kontekstna poruka koja opisuje okidač.
- **Assistant** - odgovori modela, uključujući pozive alata.
- **Tool** - rezultat alata vraćen modelu (npr., šta je `search_memory` vratio).

Duge poruke se mogu skupiti; kliknite **Proširi** / **Sažmi** da prikažete.

### Čitanje transkripata

Transkript je najvažnija stranica za podešavanje. Kada agent donese odluku sa kojom se ne slažete, pročitajte ga da biste videli:

- Šta je model **video** (User kontekstna poruka).
- Šta je model **odlučio** (Assistant pozivi alata).
- Šta je model **razmatrao** (bilo koji rezultati alata - npr. da li je agent zaista pozvao `search_memory` i da li je našao nešto pre zabrane).

Ako model dosledno pravi isti tip greške, izmenite [početni prompt](#personality-prompt) - ili koristite [Refining Prompts](#refining-prompts) iz odbijenog odobrenja.

### Reference akcija

Reference ID-jevi su prikazani kao monospace tekst (nije hyperlink):

- Komentari: ID komentara.
- Korisnici: ID korisnika.
- Bedževi: ID bedža.

Možete kopirati ID da pronađete pogođeni zapis na odgovarajućoj strani za moderaciju/administraciju.

### Šta nedostaje u Dry Run

Dry Run izvršavanja prikazuju **iste** akcije, opravdanja i ocene poverenja. Jedina razlika je **Dry Run** bedž na redu sa statusom. Reference ID-jevi za komentare / korisnike / bedževe su i dalje prikazani - agent ih jednostavno nije promenio.

### Greške

Za izvršavanja u `Error` stanju, stranica sa detaljima prikazuje osnovnu poruku o grešci. Uobičajene greške:

- **No LLM API key configured** - greška u konfiguraciji tenanta ili platforme.
- **LLM call timeout** - LLM provajder je bio spor ili nedostupan.
- **Tool dispatch failure** - agent je izabrao alat sa pogrešnim argumentima (npr. ID komentara koji više ne postoji).
- **Budget exhausted mid-run** - agentov limit je dostignut dok je izvršavanje bilo u toku. Izvršavanje je zaustavljeno.

Greške ne poništavaju delimične akcije - svi pozivi alata završeni pre greške ostaju.