Agent cost je **zasnovan na tokenima**. Svaki LLM poziv vraća broj tokena, platforma to pretvara u cente USD koristeći stopu po tokenu modela, a centi se naplaćuju protiv budžeta agenta i tenanta.

### Što se naplaćuje

- **Svi LLM pozivi**, uključujući poziv koji ne proizvede nijednu akciju alata ("agent je odlučio ne raditi ništa"). Inference se plaća čak i kada ne nastane akcija.
- **Dry-run pozivi**. Dry-run znači "ne djeluj, ali ipak pozovi LLM" - LLM poziv košta isto. Vidi [Dry-Run Mode](#dry-run-mode).
- **Replay pozivi**. Replayi su dry-run izvođenja protiv povijesnih komentara. Oni koštaju tokene. Vidi [Test Runs (Replays)](#test-runs-replays).

### Što se ne naplaćuje

- **Okidači koji nikada ne proizvedu LLM poziv.** Slučajevi "dropped-before-LLM" (prekoračen budžet, rate limited, neslaganje opsega, nevažeće naplaćivanje, sprječavanje petlje) ne koštaju tokene. Vidi [Drop Reasons](#drop-reasons).
- **Dispatch alata.** Pozivanje `pin_comment` ili bilo kojeg drugog alata samo po sebi ne troši tokene - jedino LLM round-trip troši tokene.
- **`search_memory`.** Ono je samo za čitanje i ne proizvodi vlastiti LLM round-trip.

### Trošak po izvođenju

Jedno izvođenje agenta može pozvati LLM više puta - svaki rezultat poziva alata se vraća natrag modelu kako bi on mogao pozvati još jedan alat ili završiti. Dakle, `tokensUsed` na izvođenju je zbroj kroz sve LLM round-tripove u tom izvođenju.

Najveći doprinosi trošku po izvođenju:

- **Dugi [initial prompts](#personality-prompt) i [community guidelines](#community-guidelines)** - oni se upisuju pri svakom izvođenju.
- **[Context options](#context-options)** - kontekst niti, povijest korisnika, metadata stranice. Svaki dodaje tokene.
- **Sadržaj komentara** - dugi komentari koštaju više.
- **Višestruki pozivi alata u jednom izvođenju** - rezultat svakog alata se šalje natrag modelu.
- **Čitanja memorije** - `search_memory` vraća do 25 zapisa (ograničeno na ukupno 8000 znakova sadržaja). Većina tih bajtova ulazi u sljedeći prompt.

**Max Tokens Per Trigger** (zadano 20,000) ograničava veličinu **odgovora** po LLM pozivu. Ne ograničava veličinu ulaza.

### Pretvorba tokena u cente

Platforma primjenjuje jednu stopu po tenant-paketu (`flexLLMCostCents` po `flexLLMUnit` tokena). Cijena po tokenu je na razini paketa, ne po modelu - oba dostupna modela ([GLM 5.1 and GPT-OSS Turbo](#choosing-a-model)) naplaćuju po istoj stopi na danom paketu. [Run Detail View](#run-detail-view) prikazuje trošak po izvođenju u vašoj valuti nakon što se izvođenje završi.

### Gdje se trošak bilježi

Svako izvođenje bilježi sirovi broj tokena i trošak po izvođenju. Dnevni i mjesečni zbrojevi se sabiraju na [Analytics page](#analytics-page).

### Kako čitati trošak

- **Trošak po izvođenju**: [Run Detail View](#run-detail-view) -> `Cost` polje.
- **Dnevni / mjesečni agregat**: [Analytics page](#analytics-page) -> Grafici korištenja budžeta i dnevnog troška.
- **Trošak po akciji**: također u Run Detail View, koristan za podešavanje kada je petlja alata agenta neuobičajeno dugačka.

### Vidi također

- [Choosing a Model](#choosing-a-model) - najveći utjecaj na trošak.
- [Context Options](#context-options) - odakle dolazi dodatni trošak.
- [Budgets Overview](#budgets-overview) - čvrsti ograničitelji koji sprječavaju nekontrolirani rast troškova.