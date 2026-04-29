Povijest pokretanja je zapis po agentu svakog okidača koji se pokrenuo. Dostupna je sa stranice popisa agenata putem gumba **Pokretanja**, ili izravno na `/auth/my-account/ai-agents/{agentId}/runs`.

### Što se nalazi na stranici

Paginirana tablica s jednim retkom po pokretanju:

| Column | Meaning |
|---|---|
| Date | Kada se okidač aktivirao (ili kada je odgođeni okidač pokrenut). |
| Status | **Pokrenuto**, **Uspješno**, ili **Greška**. Pored se prikazuje oznaka **Testno pokretanje** ako je pokretanje bilo u režimu suhog pokusa. |
| Cost | Trošak po izvođenju u valuti vašeg tenant-a. Prazno za izvođenja u tijeku (Pokrenuto). |
| Actions | Broj poziva alata u izvođenju. |
| Details | Gumb **Prikaži** koji otvara [Pregled detalja izvođenja](#run-detail-view). |

### Značenje statusa

- **Pokrenuto** - izvođenje je u tijeku, ili je prekinuto prije završetka. Izvođenje zaglavljeno u "Pokrenuto" neuobičajeno dugo obično predstavlja istek vremena poziva LLM-a.
- **Greška** - izvođenje je završilo, ali je negdje došlo do pogreške - poziv LLM-a je vratio pogrešku, slanje alatu nije uspjelo itd. Pregled detalja sadrži specifičnu grešku.
- **Uspješno** - izvođenje je završilo bez pogrešaka. Agent je mogao poduzeti nula, jednu ili više radnji.

### Prazno stanje

Kada agent nema pokretanja, stranica prikazuje: "No runs yet for this agent. Enabled runs appear here once a trigger fires; use Test run to preview what this agent would do against past comments."

Zadnji dio je namjeran - [tok testnog pokretanja](#test-runs-replays) je preporučeni način da popunite Povijest pokretanja na novom agentu.

### Što se ne nalazi na stranici povijesti izvođenja

- **Živi okidači koji nikada nisu dispatchani** - okidač koji je odbačen zbog budžeta, opsega ili ograničenja brzine ne pojavljuje se na ovoj stranici. Oni se prikazuju na [stranici Analytics](#analytics-page) pod "Preskočeni okidači".
- **Odobrenja** - odobrenja na čekanju za radnje poduzete u ovom izvođenju nalaze se u [inboxu za odobrenja](#approval-workflow). Radnja se u prikazu detalja izvođenja pojavljuje kao **Na čekanju odobrenja**.

### Zadržavanje podataka

Pojedinačni zapisi pokretanja čuvaju se 90 dana, nakon čega pokretanje više nije dostupno u povijesti. Troškovi i brojevi okidača nastavljaju se zbrajati u dugoročnim analitičkim sažetcima, pa [stranica Analytics](#analytics-page) i dalje prikazuje povijesne ukupne vrijednosti izvan tog razdoblja.

### Reprodukcije

Izvedbe nastale reprodukcijom (replay) su po zadanom izuzete iz prikaza aktivnih izvođenja. Stranica [Test Runs (Replays)](#test-runs-replays) je mjesto gdje ih možete vidjeti.

### Filtriranje među agentima

Tablica pokretanja je po agentu. Ne postoji prikaz pokretanja preko agenata - [stranica Analytics](#analytics-page) je sažetak preko agenata. Ako trebate pregledati pokretanja za više agenata, događaji [Webhooks](#webhooks-overview) `trigger.succeeded` i `trigger.failed` su oni koje biste prosljeđivali svom sustavu.