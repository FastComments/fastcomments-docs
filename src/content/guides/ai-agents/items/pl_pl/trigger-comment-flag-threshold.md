Uruchamia się, gdy liczba zgłoszeń komentarza osiąga **dokładnie** skonfigurowany próg.

### Wymagana konfiguracja

- **Próg zgłoszeń** - liczba całkowita >= 1. Wyzwalacz uruchamia się w momencie, gdy `flagCount === flagThreshold`. Nie uruchamia się ponownie przy kolejnych zgłoszeniach powyżej progu.

Jeśli próg wynosi 3 i trzech użytkowników zgłosi komentarz, agent uruchamia się raz przy trzecim zgłoszeniu. Czwarte, piąte czy szóste zgłoszenie **nie** spowoduje ponownego uruchomienia.

### Kontekst, który otrzymuje agent

- Zgłoszony komentarz.
- Opcjonalny kontekst wątku / historii użytkownika / strony zgodnie z konfiguracją.
- Liczba zgłoszeń znajduje się w bloku komentarza jako `Flag Count: N`.

### Ważne

- Wyzwalacz uruchamia się tylko wtedy, gdy komentarz przekracza próg od strony obsługi zgłoszeń platformy (gdzie `didIncrement === true`). Bezpośrednie zapisy w bazie danych ustawiające `flagCount` na wartość progu nie powodują jego uruchomienia; zgłoszenia powyżej progu również nie powodują ponownego uruchomienia.
- Nie zawiera informacji, kto zgłosił komentarz — zgłoszenia są dla agenta anonimowe. Jeśli chcesz sprawdzić użytkowników zgłaszających, pobierz ich z własnych danych.
- Zalecane jest *wyraźnie* opóźnienie wyzwalacza (zob. [Odroczone wyzwalacze](#trigger-deferred-delay)) dla tego wyzwalacza — zgłoszenia często pojawiają się seriami podczas gorącego wątku, a krótkie opóźnienie pozwala sytuacji się ustabilizować zanim agent podejmie działanie.

### Typowe zastosowania

- **Przegląd moderacyjny** - zgłoszony komentarz jest kanonicznym sygnałem „ludzie uważają, że może to być niewłaściwe”. [Szablon moderatora](#template-moderator) subskrybuje ten wyzwalacz domyślnie z progiem zgłoszeń równym 3.
- **Rozszerzenie kolejki pre-moderacji** - agent wykonuje wstępne przetwarzanie i albo oznacza komentarz do moderacji (przy użyciu `mark_comment_reviewed`), albo eskaluje sprawę dalej.
- **Przeciwdziałanie brigadom** - połącz ten wyzwalacz z [kontekstem historii użytkownika](#context-options) i pozwól agentowi zobaczyć wcześniejsze bany/sygnały o duplikowanej treści przed podjęciem działań.

### Zalecenia dotyczące łączenia

Zasubskrybuj **oba** `COMMENT_ADD` i `COMMENT_FLAG_THRESHOLD`, jeśli chcesz mieć agenta moderującego, który wychwytuje oczywiste przypadki od razu i ponownie ocenia wątpliwe po zgromadzeniu zgłoszeń. Oba zdarzenia wywoływane są niezależnie - agent uruchomi się dwukrotnie, jeśli oba zostaną zasubskrybowane i oba się wywołają, ale drugie uruchomienie zobaczy już stan z zgłoszeniami.

---