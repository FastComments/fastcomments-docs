Koszt agenta jest **oparty na tokenach**. Każde wywołanie LLM zwraca liczbę tokenów, platforma przelicza ją na centy USD według stawki za token danego modelu, a centy są rozliczane przeciwko budżetom agenta i tenanta.

### Co jest rozliczane

- **Wszystkie wywołania LLM**, w tym wywołanie, które skutkuje zerem akcji narzędziowych („agent zdecydował, że nic nie zrobi"). Inferencja jest płatna nawet wtedy, gdy nie skutkuje żadną akcją.
- **Wywołania dry-run**. Dry-run to „nie działaj, ale nadal wywołaj LLM" - wywołanie LLM kosztuje tyle samo. Zobacz [Tryb Dry-Run](#dry-run-mode).
- **Wywołania replay**. Replaye to uruchomienia dry-run na historycznych komentarzach. Kosztują tokeny. Zobacz [Uruchomienia testowe (Replaye)](#test-runs-replays).

### Co nie jest rozliczane

- **Wyzwalacze, które nigdy nie powodują wywołania LLM.** Przypadki odrzucone przed LLM (przekroczony budżet, limit szybkości, niezgodność zakresu, nieprawidłowe rozliczenie, zapobieganie pętli) kosztują zero tokenów. Zobacz [Powody odrzucenia](#drop-reasons).
- **Wywoływanie narzędzi.** Wywołanie `pin_comment` lub innego narzędzia samo w sobie nie kosztuje tokenów - kosztuje tylko pełna wymiana z LLM.
- **`search_memory`.** Jest tylko do odczytu i nie powoduje własnej wymiany z LLM.

### Koszt na uruchomienie

Pojedyncze uruchomienie agenta może wywołać LLM wielokrotnie - wynik każdego wywołania narzędzia jest przekazywany z powrotem do modelu, aby mógł wywołać kolejne narzędzie lub zakończyć działanie. Dlatego `tokensUsed` dla uruchomienia to suma wszystkich wymian z LLM w ramach tego uruchomienia.

Największe czynniki kosztu tokenów na uruchomienie:

- **Długie [prompty początkowe](#personality-prompt) i [wytyczne społeczności](#community-guidelines)** - trafiają do każdego uruchomienia.
- **[Opcje kontekstu](#context-options)** - kontekst wątku, historia użytkownika, metadane strony. Każdy element dodaje tokeny.
- **Sam tekst komentarza** - długie komentarze kosztują więcej.
- **Wiele wywołań narzędzi w jednym uruchomieniu** - wynik każdego narzędzia jest wysyłany z powrotem do modelu.
- **Odczyty pamięci** - `search_memory` zwraca do 25 rekordów (limit łącznie 8000 znaków treści). Większość tych bajtów trafia do kolejnego promptu.

**Max Tokens Per Trigger** (domyślnie 20 000) ogranicza rozmiar **odpowiedzi** na wywołanie LLM. Nie ogranicza rozmiaru wejścia.

### Przeliczanie tokenów na centy

Platforma stosuje jedną stawkę na pakiet tenanta (`flexLLMCostCents` za `flexLLMUnit` tokenów). Koszt za token jest na poziomie pakietu, a nie modelu - oba dostępne modele ([GLM 5.1 i GPT-OSS Turbo](#choosing-a-model)) są rozliczane po tej samej stawce w ramach danego pakietu. [Widok szczegółów uruchomienia](#run-detail-view) pokazuje koszt uruchomienia w Twojej walucie po jego zakończeniu.

### Gdzie zapisywany jest koszt

Każde uruchomienie zapisuje surową liczbę tokenów i koszt uruchomienia. Sumy dzienne i miesięczne są agregowane na [stronie Analytics](#analytics-page).

### Jak odczytać koszt

- **Koszt na uruchomienie**: [Widok szczegółów uruchomienia](#run-detail-view) -> pole `Cost`.
- **Suma dzienna / miesięczna**: [strona Analytics](#analytics-page) -> wykresy Budget usage i Daily cost.
- **Koszt na akcję**: również w Widoku szczegółów uruchomienia, przydatny do strojenia, gdy pętla narzędzi agenta jest wyjątkowo długa.

### Zobacz także

- [Wybór modelu](#choosing-a-model) - większa dźwignia wpływu na koszt.
- [Opcje kontekstu](#context-options) - skąd pochodzi dodatkowy koszt.
- [Przegląd budżetów](#budgets-overview) - twarde limity zapobiegające niekontrolowanym kosztom.