Trošak agenta je **zasnovan na tokenima**. Svaki LLM poziv vraća broj tokena; platforma ga konvertuje u cente (USD) koristeći stopu po tokenu modela, i ti centi se naplaćuju na teret budžeta agenta i tenanta.

### Šta se naplaćuje

- **Svi LLM pozivi**, uključujući poziv koji ne proizvodi nijednu akciju alata ("agent je odlučio da ne radi ništa"). Inferencija se plaća čak i kada nema rezultujuće akcije.
- **Dry-run pozivi**. Dry-run je "ne delovati, ali ipak pozvati LLM" - LLM poziv košta isto. Pogledajte [Način Dry-Run](#dry-run-mode).
- **Replay pozivi**. Replay su dry-run pokretanja protiv istorijskih komentara. Koštaju tokene. Pogledajte [Test Runs (Replays)](#test-runs-replays).

### Šta se ne naplaćuje

- **Okidači koji nikada ne generišu LLM poziv.** Slučajevi odbačeni pre LLM-a (prekoračenje budžeta, ograničenje po brzini, neusklađenost opsega, nevažeće fakturisanje, sprečavanje petlje) ne koštaju tokene. Pogledajte [Drop Reasons](#drop-reasons).
- **Pozivanje alata.** Pozivanje `pin_comment` ili bilo kog drugog alata samo po sebi ne košta tokene - samo LLM rund-trip troši tokene.
- **`search_memory`.** On je samo za čitanje i ne proizvodi sopstvenu LLM rund-trip.

### Trošak po pokretanju

Jedno pokretanje agenta može pozvati LLM više puta - svaki rezultat poziva alata vraća se modelu kako bi mogao ili pozvati drugi alat ili završiti. Dakle, `tokensUsed` za jedno pokretanje je zbir preko svih LLM rund-tripova u tom pokretanju.

Najveći faktori koji doprinose trošku tokena po pokretanju:

- **Dugi [početni promptovi](#personality-prompt) i [smernice zajednice](#community-guidelines)** - oni se pojavljuju u svakom pokretanju.
- **[Opcije konteksta](#context-options)** - kontekst niti, istorija korisnika, metapodaci stranice. Svaki dodaje tokene.
- **Tekst samog komentara** - dugi komentari koštaju više.
- **Više poziva alata u jednom pokretanju** - poruka sa rezultatom svakog alata se šalje nazad modelu.
- **Čitanja memorije** - `search_memory` vraća do 25 zapisa (ograničeno na ukupno 8000 karaktera sadržaja). Većina tih bajtova ide u sledeći prompt.

**Max Tokens Per Trigger** (default 20,000) ograničava veličinu **odgovora** po LLM pozivu. Ne ograničava veličinu ulaza.

### Konverzija tokena u cente

Platforma primenjuje jedinstvenu stopu po tenant-paketu (`flexLLMCostCents` po `flexLLMUnit` tokena). Cena po tokenu je na nivou paketa, ne po modelu - oba dostupna modela ([GLM 5.1 and GPT-OSS Turbo](#choosing-a-model)) naplaćuju se istom stopom za dati paket. [Pregled detalja pokretanja](#run-detail-view) prikazuje cenu po pokretanju u vašoj valuti kada se pokretanje završi.

### Gde se trošak beleži

Svako pokretanje beleži svoj sirovi broj tokena i trošak po pokretanju. Dnevni i mesečni zbiri se sabiraju na [Stranica analitike](#analytics-page).

### Kako čitati trošak

- **Trošak po pokretanju**: [Pregled detalja pokretanja](#run-detail-view) -> polje `Cost`.
- **Dnevni / mesečni zbir**: [Stranica analitike](#analytics-page) -> grafici korišćenja budžeta i dnevnog troška.
- **Trošak po akciji**: takođe u Pregledu detalja pokretanja, koristan za podešavanje kada je petlja alata agenta neuobičajeno duga.

### Pogledajte i

- [Izbor modela](#choosing-a-model) - najveći faktor koji utiče na trošak.
- [Opcije konteksta](#context-options) - odakle dolazi dodatni trošak.
- [Pregled budžeta](#budgets-overview) - čvrsti limiti koji sprečavaju nekontrolisane troškove.