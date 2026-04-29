Klikom na **View** na retku u [Povijest pokretanja](#run-history) otvara se stranica s detaljima za pojedino pokretanje. Ovo je mjesto gdje čitate razmišljanje agenta i procjenjujete njegove odluke.

### Gornji dio: sažetak izvršavanja

- **Agent** - koji je agent pokrenut.
- **When** - vremenska oznaka.
- **Status** - Started / Success / `Error`, plus the **Probni način** značka ako je primjenjivo.
- **Cost** - trošak po pokretanju u valuti vašeg najmodavca.
- **Cost per action** - trošak podijeljen s brojem ne-pending radnji, koristan za uočavanje neuobičajeno skupih pokretanja.

### Poduzete radnje

Popis, redom, svakog poziva alata koji je pokretanje izvršilo. Svaki unos prikazuje:

- **Action label** - "Wrote a comment", "Marked a comment as spam", "Banned a user" i tako dalje. Oznaka se preslikava iz enumeracije tipova akcija.
- **Reference ID** - zahvaćeni ID komentara, korisnika ili značke, prikazan monospace tekstom (nije hiperlink).
- **Agent reasoning** - opravdanje koje je agent priložio uz poziv.
- **Confidence** - agentova samoprocijenjena pouzdanost, prikazana kao postotak.
- **Pending approval** značka - ako je akcija stavljena u red u [pretinac odobrenja](#approval-workflow) umjesto da je izvršena.

Ako pokretanje nije izvršilo nijednu radnju, odjeljak prikazuje: "Tijekom ovog izvršavanja nisu poduzete nikakve radnje."

### LLM transkript

Ispod radnji, puni transkript razgovora agenta s LLM-om:

- **System** - sistemski prompt (platform suffix + vaš početni prompt + smjernice zajednice).
- **User** - poruka konteksta koja opisuje okidač.
- **Assistant** - odgovori modela, uključujući pozive alata.
- **Tool** - rezultat alata vraćen modelu (npr. što je `search_memory` vratio).

Duge poruke su sažimljive; kliknite **Proširi** / **Sažmi** za prikaz.

### Čitanje transkripata

Transkript je najvažnija stranica za podešavanje. Kad agent donese odluku s kojom se ne slažete, pročitajte je unatrag da vidite:

- Što je model **vidio** (poruka konteksta User).
- Što je model **odlučio** (Assistant pozivi alata).
- Što je model **razmotrio** (bilo koji rezultati alata - npr. je li agent zapravo pozvao `search_memory` i je li nešto pronašao prije zabrane).

Ako model dosljedno pravi istu vrstu pogreške, uredite [početni prompt](#personality-prompt) - ili upotrijebite [Usavršavanje promptova](#refining-prompts) iz odbijenog odobrenja.

### Referentne oznake radnji

Referentni ID-ovi prikazuju se monospace tekstom (nisu hiperlinkovi):

- Comments: ID komentara.
- Users: ID korisnika.
- Badges: ID značke.

Možete kopirati ID da biste pronašli zahvaćeni zapis na relevantnoj stranici za moderiranje/administraciju.

### Što nedostaje u probnom načinu

Probna pokretanja prikazuju **iste** radnje, opravdanja i ocjene pouzdanosti. Jedina razlika je značka **Probni način** na retku statusa. Referentni ID-ovi za komentare / korisnike / značke su i dalje prikazani - agent ih jednostavno nije utjecao.

### Pogreške

Za pokretanja u stanju `Error`, stranica s detaljima prikazuje temeljnu poruku o pogrešci. Uobičajene pogreške:

- **No LLM API key configured** - pogrešna konfiguracija najmodavca ili platforme.
- **LLM call timeout** - davatelj LLM-a je bio spor ili nedostupan.
- **Tool dispatch failure** - agent je odabrao alat s lošim argumentima (npr. ID komentara koji više ne postoji).
- **Budget exhausted mid-run** - agentov limit je dosegnut dok je pokretanje bilo u tijeku. Pokretanje je zaustavljeno.

Pogreške ne poništavaju djelomične radnje - svi pozivi alata dovršeni prije pogreške ostaju.