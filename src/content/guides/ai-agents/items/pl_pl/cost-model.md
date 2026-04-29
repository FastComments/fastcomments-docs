Koszt agenta jest **oparty na tokenach**. Każde wywołanie LLM zwraca liczbę tokenów, platforma konwertuje to na centy USD używając stawki za token modelu, a centy są obciążane budżetami agenta i najemcy.

### Co jest rozliczane

- **Wszystkie wywołania LLM**, w tym wywołanie, które nie powoduje żadnych akcji narzędziowych ("agent postanowił nic nie robić"). Inferencja jest płatna nawet gdy nie ma żadnego rezultatu działania.
- **Wywołania w trybie dry-run**. Dry-run oznacza "nie wykonywać działań, ale nadal wywołać LLM" - wywołanie LLM kosztuje tyle samo. Zobacz [Tryb Dry-Run](#dry-run-mode).
- **Wywołania powtórkowe (replay)**. Powtórki są uruchomieniami w trybie dry-run przeciw historycznym komentarzom. Kosztują tokeny. Zobacz [Uruchomienia testowe (powtórki)](#test-runs-replays).

### Co nie jest rozliczane

- **Wyzwalacze, które nigdy nie powodują wywołania LLM.** Przypadki odrzucone przed wywołaniem LLM (przekroczony budżet, ograniczenia szybkości, niezgodność zakresu, nieprawidłowe rozliczenia, zapobieganie pętli) nie kosztują tokenów. Zobacz [Powody odrzucenia](#drop-reasons).
- **Wywołanie narzędzia.** Wywołanie `pin_comment` lub jakiegokolwiek innego narzędzia samo w sobie nie kosztuje tokenów — tylko runda LLM jest płatna.
- **`search_memory`.** Jest tylko do odczytu i nie powoduje własnej rundy LLM.

### Koszt na uruchomienie

Pojedyncze uruchomienie agenta może wywołać LLM wielokrotnie — wynik każdego wywołania narzędzia jest wprowadzany z powrotem do modelu, aby mógł on wywołać kolejne narzędzie lub zakończyć działanie. Zatem `tokensUsed` dla uruchomienia to suma wszystkich rund LLM w tym uruchomieniu.

Największe czynniki wpływające na koszt tokenów na uruchomienie:

- **Długie [początkowe prompty](#personality-prompt) i [wytyczne społeczności](#community-guidelines)** — są dołączane przy każdym uruchomieniu.
- **[Opcje kontekstu](#context-options)** — kontekst wątku, historia użytkownika, metadane strony. Każdy z nich dodaje tokeny.
- **Sam tekst komentarza** — długie komentarze kosztują więcej.
- **Wielokrotne wywołania narzędzi w jednym uruchomieniu** — wynik każdego narzędzia jest wysyłany z powrotem do modelu.
- **Odczyty z pamięci** — `search_memory` zwraca do 25 rekordów (ograniczone do 8000 znaków całkowitej zawartości). Większość tych bajtów trafia do następnego promptu.

**Maksymalna liczba tokenów na wyzwalacz** (domyślnie 20 000) ogranicza rozmiar **odpowiedzi** na wywołanie LLM. Nie ogranicza rozmiaru wejścia.

### Konwersja tokenów na centy

Platforma stosuje jedną stawkę na pakiet najemcy (`flexLLMCostCents` za `flexLLMUnit` tokenów). Koszt za token jest określany na poziomie pakietu, nie na modelu — oba dostępne modele ([GLM 5.1 and GPT-OSS Turbo](#choosing-a-model)) są rozliczane tą samą stawką dla danego pakietu. [Widok szczegółów uruchomienia](#run-detail-view) pokazuje koszt na uruchomienie w Twojej walucie po zakończeniu uruchomienia.

### Gdzie rejestrowany jest koszt

Każde uruchomienie zapisuje surową liczbę tokenów i koszt dla uruchomienia. Suma dzienna i miesięczna jest agregowana na [Stronie analityki](#analytics-page).

### Jak czytać koszty

- **Koszt na uruchomienie**: [Widok szczegółów uruchomienia](#run-detail-view) -> pole `Cost`.
- **Agregat dzienny / miesięczny**: [Strona analityki](#analytics-page) -> wykresy użycia budżetu i dziennego kosztu.
- **Koszt na akcję**: również na [Widoku szczegółów uruchomienia](#run-detail-view), przydatne do optymalizacji, gdy pętla narzędzi agenta jest wyjątkowo długa.

### Zobacz także

- [Wybór modelu](#choosing-a-model) - główny czynnik wpływający na koszt.
- [Opcje kontekstu](#context-options) - skąd pochodzi dodatkowy koszt.
- [Przegląd budżetów](#budgets-overview) - twarde limity, które zapobiegają wymknięciu się kosztów.