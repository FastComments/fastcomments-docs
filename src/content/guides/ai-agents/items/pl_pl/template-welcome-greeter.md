**Template ID:** `welcome_greeter`

Powitalny Greeter odpowiada serdecznie osobom komentującym po raz pierwszy. Jest to szablon o najniższym ryzyku (brak narzędzi destrukcyjnych) i dobry pierwszy agent do wdrożenia na żywo.

### Wyzwalacze

- **Nowy użytkownik publikuje swój pierwszy komentarz na tej witrynie** (`NEW_USER_FIRST_COMMENT`).

To zdarzenie wywoływane jest dokładnie raz na użytkownika, więc agent nie może się zapętlić. Zobacz [Wyzwalacz: Pierwszy komentarz nowego użytkownika](#trigger-new-user-first-comment).

### Dozwolone narzędzia

- [`write_comment`](#tools-overview)

To jedyne narzędzie — agent dosłownie nie może moderować, głosować, banować ani wysyłać DM.

### Zalecane dodatki przed uruchomieniem na żywo

- **Ustaw nazwę wyświetlaną (Display name)** na coś zachęcającego — "Community Bot", maskotkę strony lub nazwę Twojej marki. Nazwa wyświetlana to to, co czytelnicy widzą przypięte do powitalnej odpowiedzi.
- **Zaznacz „Dołącz tytuł strony, podtytuł, opis i metatagi”** w [Opcje kontekstu](#context-options). Odpowiedzi greetera stają się zauważalnie lepsze, gdy może odnosić się do tego, o czym faktycznie jest strona.
- **Rozważ ograniczenia dotyczące lokalizacji (locale)**, jeśli działasz w wielu językach. Powitalna odpowiedź w nieodpowiednim języku jest bardziej krępująca niż brak odpowiedzi. Zobacz [Zakres: Filtry URL i lokalizacji](#scope-url-locale).

### Dlaczego nie są potrzebne zatwierdzenia

Agent jedynie pisze nowe komentarze i działa tylko na jednorazowym wyzwalaczu. W najgorszym wypadku: niezręczne powitanie. Nie ma żadnej destrukcyjnej akcji wymagającej blokady. Większość operatorów uruchamia ten szablon bez żadnych zatwierdzeń, gdy testy próbne (dry-run) wyglądają dobrze.