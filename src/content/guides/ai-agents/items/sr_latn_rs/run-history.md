Run History je dnevnik po agentu koji beleži svaki pokrenuti okidač. Dostupan je sa stranice sa listom agenata preko dugmeta **Runs**, ili direktno na `/auth/my-account/ai-agents/{agentId}/runs`.

### Šta se nalazi na stranici

Paginirana tabela sa jednim redom po pokretanju:

| Column | Meaning |
|---|---|
| Datum | Kada je okidač pokrenut (ili kada je odloženi okidač izvršen). |
| Status | **Pokrenuto**, **Uspešno**, ili **Greška**. Pored toga se prikazuje oznaka **Dry Run** ako je pokretanje bilo u probnom režimu. |
| Trošak | Trošak po pokretanju u valuti vašeg tenant-a. Prazno za pokretanja koja su u toku (Pokrenuto). |
| Radnje | Broj poziva alata u pokretanju. |
| Detalji | Dugme **View** koje otvara [Pregled detalja pokretanja](#run-detail-view). |

### Značenje statusa

- **Pokrenuto** - pokretanje je u toku, ili je prekinuto pre završetka. Pokretanje koje duže vreme ostane u stanju "Pokrenuto" obično ukazuje na istekao timeout poziva LLM-a.
- **Greška** - pokretanje je završeno, ali je negde nastao problem - LLM poziv je vratio grešku, dispatch alata je pao itd. Detaljni pogled sadrži specifičnu grešku.
- **Uspešno** - pokretanje je završeno bez grešaka. Agent je mogao da ne preduzme nikakvu radnju, jednu ili više radnji.

### Prazno stanje

Kada agent nema pokretanja, stranica prikazuje: "No runs yet for this agent. Enabled runs appear here once a trigger fires; use Test run to preview what this agent would do against past comments."

Ovaj poslednji deo je nameran — [tok probnog pokretanja](#test-runs-replays) je preporučeni način da popunite Run History za novog agenta.

### Šta NIJE na stranici istorije pokretanja

- **Uživo okidači koji nikada nisu poslati** - okidač koji je odbijen zbog budžeta, opsega ili ograničenja frekvencije se ne pojavljuje na ovoj stranici. Oni se pojavljuju na [Analytics page](#analytics-page) pod "Triggers skipped".
- **Odobrenja** - nerešena odobrenja za radnje preduzete u ovom pokretanju nalaze se u [approvals inbox](#approval-workflow). Radnja se prikazuje u prikazu detalja pokretanja kao **Pending approval**.

### Zadržavanje podataka

Pojedinačni zapisi o pokretanjima se čuvaju 90 dana, nakon čega pokretanje više nije dostupno u istoriji. Troškovi i brojevi okidača se i dalje akumuliraju u dugoročnim analitičkim izveštajima, tako da [Analytics page](#analytics-page) i dalje prikazuje istorijske zbirne podatke van tog perioda.

### Reprodukcije

Pokretanja nastala iz reprodukcija su po defaultu isključena iz prikaza uživo. Stranica [Test Runs (Replays)](#test-runs-replays) je mesto gde možete videti ta pokretanja.

### Filtriranje preko agenata

Tabela pokretanja je po agentu. Ne postoji prikaz pokretanja preko više agenata - [Analytics page](#analytics-page) je zbirni prikaz preko agenata. Ako treba da pregledate pokretanja preko više agenata, događaji [Webhooks](#webhooks-overview) `trigger.succeeded` i `trigger.failed` su oni koje treba proslediti u vaš sistem.