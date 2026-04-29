---
Trošak agenta je **zasnovan na tokenima**. Svaki poziv LLM-a vraća broj tokena, platforma to prevodi u centi USD koristeći stopu po tokenu modela, i centi se naplaćuju protiv budžeta agenta i zakupca.

### Šta se naplaćuje

- **Svi pozivi LLM-a**, uključujući poziv koji ne proizvodi nijednu akciju alatke ("agent je odlučio da ne radi ništa"). Inferencija se plaća čak i kada ne dođe do akcije.
- **Dry-run pozivi**. Dry-run znači "ne deluj, ali ipak pozovi LLM" - poziv LLM-a košta isto. Vidi [Dry-Run Mode](#dry-run-mode).
- **Replay pozivi**. Replay su dry-run pokretanja nad istorijskim komentarima. Oni koštaju tokene. Vidi [Test Runs (Replays)](#test-runs-replays).

### Šta se ne naplaćuje

- **Okidači koji nikada ne proizvode poziv LLM-u.** Slučajevi odbacivanja pre LLM-a (preko budžeta, ograničenje učestalosti, neusklađenost opsega, neispravno naplaćivanje, sprečavanje petlje) ne koštaju tokene. Vidi [Drop Reasons](#drop-reasons).
- **Pozivanje alatki.** Pozivanje `pin_comment` ili bilo koje druge alatke samo po sebi ne košta tokene — tokene troši samo LLM runda.
- **`search_memory`.** To je samo za čitanje i ne pokreće svoju vlastitu LLM rundu.

### Trošak po izvršavanju

Jedno izvršavanje agenta može pozvati LLM višestruko — svaki rezultat poziva alatke se prosleđuje nazad modelu tako da on može pozvati drugu alatku ili završiti. Dakle, `tokensUsed` za jedno izvršavanje je zbir kroz sve LLM runde u tom izvršavanju.

Najveći doprinosi trošku po izvršavanju:

- **Dugi [initial prompts](#personality-prompt) i [community guidelines](#community-guidelines)** - oni se ubacuju pri svakom izvršavanju.
- **[Context options](#context-options)** - kontekst teme, istorija korisnika, meta-podaci stranice. Svaki dodaje tokene.
- **Sam tekst komentara** - dugi komentari koštaju više.
- **Višestruki pozivi alatki u jednom izvršavanju** - poruka sa rezultatom svake alatke se šalje nazad modelu.
- **Čitanja iz memorije** - `search_memory` vraća do 25 zapisa (ograničeno na ukupno 8000 znakova sadržaja). Većina tih bajtova ide u sledeći prompt.

**Max Tokens Per Trigger** (podrazumijevano 20,000) ograničava veličinu **odgovora** po LLM pozivu. Ne ograničava veličinu ulaza.

### Pretvaranje tokena u cente

Platforma primenjuje jedinstvenu stopu po paketima zakupca (`flexLLMCostCents` po `flexLLMUnit` tokena). Cena po tokenu je na nivou paketa, a ne po modelu — oba dostupna modela ([GLM 5.1 and GPT-OSS Turbo](#choosing-a-model)) fakturišu po istoj stopi na datom paketu. [Run Detail View](#run-detail-view) prikazuje trošak po izvršavanju u vašoj valuti nakon što se izvršavanje završi.

### Gdje se trošak bilježi

Svako izvršavanje beleži svoj sirovi broj tokena i trošak po izvršavanju. Dnevni i mjesečni totalni iznosi sabiraju se na [Analytics page](#analytics-page).

### Kako tumačiti trošak

- **Trošak po izvršavanju**: [Run Detail View](#run-detail-view) -> polje `Cost`.
- **Dnevni / mjesečni zbir**: [Analytics page](#analytics-page) -> grafikoni korišćenja budžeta i dnevnog troška.
- **Trošak po akciji**: takođe na Run Detail View, koristan za podešavanje kada je petlja alatki agenta neuobičajeno duga.

### Pogledajte takođe

- [Choosing a Model](#choosing-a-model) - najveći faktor koji utiče na trošak.
- [Context Options](#context-options) - odakle dolazi dodatni trošak.
- [Budgets Overview](#budgets-overview) - tvrde granice koje sprečavaju nekontrolisani trošak.

---