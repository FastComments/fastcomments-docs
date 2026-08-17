Agent cost is **token-based**. Every LLM call returns a token count, the platform converts that to USD cents using the model's per-token rate, and the cents are billed against the agent's and tenant's budgets.

### Co jest rozliczane

- **Wszystkie wywołania LLM**, w tym wywołanie, które nie generuje żadnych akcji narzędziowych („agent postanowił nic nie robić”). Inference jest płatna nawet gdy nie powstaje żadna akcja.
- **Wywołania w trybie dry‑run**. Dry‑run to „nie wykonuj, ale nadal wywołaj LLM” – koszt wywołania LLM jest taki sam. Zobacz [Dry-Run Mode](#dry-run-mode).
- **Wywołania replay**. Replay to uruchomienia dry‑run na historycznych komentarzach. Kosztują tokeny. Zobacz [Test Runs (Replays)](#test-runs-replays).

### Co nie jest rozliczane

- **Wyzwalacze, które nigdy nie generują wywołania LLM.** Przypadki odrzucone przed LLM (przekroczony budżet, limit szybkości, niezgodność zakresu, nieprawidłowe rozliczenie, zapobieganie pętli) kosztują zero tokenów. Zobacz [Drop Reasons](#drop-reasons).
- **Wysyłanie narzędzi.** Wywołanie `pin_comment` lub dowolnego innego narzędzia nie kosztuje tokenów – kosztuje jedynie podróż LLM w obie strony.
- **`search_memory`.** Jest tylko do odczytu i nie generuje własnej podróży LLM.

### Koszt na uruchomienie

Jedno uruchomienie agenta może wywołać LLM wiele razy – wynik każdego wywołania narzędzia jest zwracany do modelu, który może wywołać kolejne narzędzie lub zakończyć działanie. Dlatego `tokensUsed` w uruchomieniu jest sumą wszystkich podróży LLM w tym uruchomieniu.

Największe czynniki wpływające na koszt tokenów na uruchomienie:

- **Długie [początkowe podpowiedzi](#personality-prompt) i [wytyczne społeczności](#community-guidelines)** – pojawiają się w każdym uruchomieniu.
- **[Opcje kontekstu](#context-options)** – kontekst wątku, historia użytkownika, metadane strony. Każda z nich dodaje tokeny.
- **Sam tekst komentarza** – długie komentarze kosztują więcej.
- **Wiele wywołań narzędzi w jednym uruchomieniu** – wynik każdego narzędzia jest wysyłany z powrotem do modelu.
- **Odczyty pamięci** – `search_memory` zwraca do 25 rekordów (ograniczone do 8000 znaków łącznej treści). Większość tych bajtów trafia do kolejnej podpowiedzi.

**Maksymalna liczba tokenów na wyzwalacz** (domyślnie 20 000) ogranicza rozmiar **odpowiedzi** na wywołanie LLM. Nie ogranicza rozmiaru wejścia.

### Konwersja tokenów na centy

Platforma stosuje jedną stawkę na pakiet najemcy (`flexLLMCostCents` za `flexLLMUnit` tokenów). Koszt za token jest określany na poziomie pakietu, a nie modelu – oba dostępne modele ([GLM 5.1 i GPT‑OSS Turbo](#choosing-a-model)) rozliczane są według tej samej stawki w ramach danego pakietu. [Run Detail View](#run-detail-view) wyświetla koszt na uruchomienie w Twojej walucie po zakończeniu uruchomienia.

### Gdzie koszt jest rejestrowany

Każde uruchomienie zapisuje surową liczbę tokenów i koszt na uruchomienie. Dzienna i miesięczna suma jest agregowana na [Analytics page](#analytics-page).

### Jak odczytywać koszt

- **Koszt na uruchomienie**: [Run Detail View](#run-detail-view) → pole `Cost`.
- **Agregat dzienny / miesięczny**: [Analytics page](#analytics-page) → wykresy użycia budżetu oraz dziennego kosztu.
- **Koszt na akcję**: również w Run Detail View, przydatny przy optymalizacji, gdy pętla narzędziowa agenta jest wyjątkowo długa.

### Zobacz także

- [Choosing a Model](#choosing-a-model) – największy czynnik wpływający na koszt.
- [Context Options](#context-options) – skąd pochodzi dodatkowy koszt.
- [Budgets Overview](#budgets-overview) – twarde limity zapobiegające niekontrolowanemu wzrostowi kosztów.