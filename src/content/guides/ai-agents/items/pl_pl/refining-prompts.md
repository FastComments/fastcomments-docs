**Doprecyzuj prompt** to przepływ pracy służący do edycji [początkowego promptu](#personality-prompt) agenta w odpowiedzi na konkretne decyzje, z którymi się nie zgadzasz. Uruchamia się go ze [skrzynki zatwierdzeń](#approval-workflow).

### Kiedy z niego korzystać

Gdy wielokrotnie odrzucasz ten sam rodzaj zatwierdzenia — „agent ciągle chce banować ludzi za używanie mocnego języka bez wskazanego celu” — prompt agenta jest dźwignią, którą możesz to naprawić. Doprecyzuj prompt to przewodnik, który pozwala:

1. Wybrać konkretne zatwierdzenie reprezentujące złą decyzję.
2. Edytować prompt z pełnym kontekstem tego, co agent zrobił i dlaczego.
3. Zapisz nowy prompt w agencie.

Efektem jest agent, który w przyszłości raczej nie podejmie tej samej decyzji.

### Uruchamianie przepływu

Ze skrzynki zatwierdzeń pod `/auth/my-account/ai-agent-approvals`:

1. Otwórz **odrzucone** zatwierdzenie. Trasa odrzuca wszystko poza REJECTED — zatwierdzenia w stanie pending i execution-failed nie są kwalifikowalne.
2. Kliknij **Doprecyzuj prompt**.

Zostaniesz przeniesiony do interfejsu prompt-refine pod adresem `/auth/my-account/ai-agent-approvals/:approvalId/refine-prompt`.

### Co pokazuje strona

- **Zatwierdzenie** - `toolName` agenta i `justification` dla odrzuconej decyzji (pełny zapis LLM nie jest tutaj pokazywany).
- **Bieżący prompt** - zapisany [początkowy prompt](#personality-prompt) agenta.
- **Pole opinii** - wpisujesz **opinię** opisującą, co powinno się zmienić (do 2000 znaków). LLM generuje proponowany nowy prompt na podstawie twojej opinii.
- **Jednolity diff w linii** - pojedynczy inline diff między bieżącym a proponowanym promptem (czerwony oznacza usunięte, zielony oznacza dodane).

Kontekst zatwierdzenia pozostaje przypięty na górze, abyś mógł(-a) odnosić się do „przypadku, który poprawiasz” podczas edycji.

### Zapis

Zapis aktualizuje pole `initialPrompt` agenta. Poprzednie uruchomienia (i poprzednie zatwierdzenia) nie są ponownie wykonywane wstecznie — nowy prompt wpływa tylko na przyszłe wyzwalacze. Jeśli chcesz sprawdzić, czy nowy prompt naprawia problem, uruchom [uruchomienie testowe / odtworzenie](#test-runs-replays) dla ostatnich 7 dni i sprawdź, czy nowy prompt nadal generowałby odrzucone zatwierdzenie.

### Czego ten przepływ nie robi

- Nie edytuje **wytycznych społeczności** — to pole ma własny edytor na głównym formularzu edycji agenta.
- Nie edytuje **wyzwalaczy**, **dozwolonych narzędzi** ani **zatwierdzania** — te elementy pozostają na głównym formularzu edycji.
- Nie wersjonuje promptu z możliwością wycofania. Poprzedni prompt nie jest przechowywany w oddzielnej kolekcji historii. Jeśli potrzebujesz wycofać zmiany, skopiuj bieżący prompt do własnego systemu śledzenia przed edycją.

### Dlaczego łączyć doprecyzowanie z odtworzeniem

Edycja promptu bez przetestowania rezultatu opiera się na wierze. Zalecany cykl:

1. Odrzuć zatwierdzenie.
2. Doprecyzuj prompt.
3. Uruchom [uruchomienie testowe](#test-runs-replays) dla ostatnich 7 dni.
4. Sprawdź kartę **Deltas**. Czy nowy prompt przeniósł złą decyzję ze stanu "zrobiłby" do "nie zrobiłby"? Czy przypadkowo nie przeniósł też dobrych decyzji?
5. Iteruj.

Trzy lub cztery cykle doprecyzowania + odtworzenia zwykle wystarczą, aby uzyskać stabilny prompt dla agenta moderującego.

### Alternatywa: bezpośrednia edycja

Nie musisz korzystać z funkcji "Doprecyzuj prompt" — możesz też po prostu edytować agenta na głównym formularzu edycji. Jedyną zaletą funkcji "Doprecyzuj prompt" jest to, że przypina konkretny nieudany przypadek, dzięki czemu nie tracisz z oczu, co poprawiasz dla.

---