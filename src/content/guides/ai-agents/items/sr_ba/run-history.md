Run History je dnevnik po agentu koji beleži svaki okidač koji je pokrenut. Dostupna je sa stranice liste agenata putem dugmeta **Izvršavanja**, ili direktno na `/auth/my-account/ai-agents/{agentId}/runs`.

### Šta se nalazi na stranici

Paginirana tabela sa jednim redom po izvršavanju:

| Column | Meaning |
|---|---|
| Date | Kada je okidač aktiviran (ili kada je odloženi okidač izvršen). |
| Status | **Pokrenuto**, **Uspjeh**, ili **Greška**. Pored toga se prikazuje oznaka **Probno izvršavanje** ako je izvršavanje bilo u probnom režimu. |
| Cost | Trošak po izvršavanju u valuti vašeg tenant-a. Prazno za izvršavanja u toku (Pokrenuto). |
| Actions | Broj poziva alata tokom izvršavanja. |
| Details | Dugme **Pregled** koje otvara [Prikaz detalja izvršavanja](#run-detail-view). |

### Značenja statusa

- **Pokrenuto** - izvršavanje je u toku, ili je prekinuto prije završetka. Izvršavanje koje dugo ostane u statusu "Pokrenuto" obično predstavlja isteka vremena pri pozivu LLM-a.
- **Greška** - izvršavanje je završeno ali je negdje nastala greška - poziv LLM-a je vratio grešku, izvršenje alata nije uspjelo, itd. Prikaz detalja sadrži specifičnu grešku.
- **Uspjeh** - izvršavanje je završeno bez greške. Agent je mogao preduzeti nula, jednu ili više akcija.

### Prazno stanje

Kada agent nema izvršavanja, stranica prikazuje: "Još nema izvršavanja za ovog agenta. Omogućena izvršavanja će se pojaviti ovdje kada okidač bude aktiviran; koristite Test izvršavanje da pregledate šta bi ovaj agent uradio nad prošlim komentarima."

Ta posljednja napomena je namjerna - [tok test izvršavanja](#test-runs-replays) je preporučeni način da popunite Historiju izvršavanja za novog agenta.

### Šta se ne nalazi na stranici historije izvršavanja

- **Aktivni okidači koji nikada nisu poslati na izvršenje** - okidač koji je odbijen zbog budžeta, opsega ili ograničenja brzine se ne pojavljuje na ovoj stranici. Oni se pojavljuju na [Stranici analitike](#analytics-page) pod "Preskočeni okidači".
- **Odobrenja** - na čekanju odobrenja za akcije preuzete u ovom izvršavanju nalaze se u [sandučetu za odobrenja](#approval-workflow). Akcija se u prikazu detalja izvršavanja pojavljuje kao **Na čekanju odobrenja**.

### Zadržavanje

Pojedinačni zapisi izvršavanja se čuvaju 90 dana, nakon čega izvršavanje nestaje iz historije. Troškovi i brojevi okidača nastavljaju se sabirati u dugoročnim analitičkim sažecima, tako da [Stranica analitike](#analytics-page) i dalje prikazuje historijske ukupne iznose izvan tog perioda.

### Reprodukcije

Izvršavanja nastala reprodukcijama su po defaultu isključena iz prikaza aktivnih izvršavanja. Stranica [Test izvršavanja (Reprodukcije)](#test-runs-replays) je mjesto gdje ih možete vidjeti.

### Filtriranje među agentima

Tabela izvršavanja je po agentu. Ne postoji prikaz izvršavanja preko više agenata - [Stranica analitike](#analytics-page) je sažetak za više agenata. Ako trebate pregledati izvršavanja preko više agenata, [Webhooks](#webhooks-overview) događaji `trigger.succeeded` i `trigger.failed` su oni koje biste prosledili u vaš sistem.