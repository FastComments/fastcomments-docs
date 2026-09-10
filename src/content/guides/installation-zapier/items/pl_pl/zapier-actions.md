## Akcje i Wyszukiwania

Akcje tworzą dane w FastComments; wyszukiwania odszukują dane, aby późniejszy krok mógł je wykorzystać. Każda akcja wywołuje
REST API FastComments i zużywa taką samą liczbę kredytów API, jaką kosztowałoby wywołanie z własnego kodu: jeden
kredyt za wywołanie, chyba że zaznaczono inaczej.

## Utwórz komentarz

Publikuje komentarz na stronie.

| Pole | Wymagane | Uwagi |
|------|----------|-------|
| ID URL strony | Tak | ID URL używany przez widżet komentarzy na stronie. Komentarze są grupowane według niego. |
| URL strony | Tak | Pełny URL strony, używany w e‑mailach powiadamiających. |
| Komentarz | Tak | Treść komentarza w formacie FastComments markdown. |
| Nazwa komentującego | Tak | Nazwy są unikalne dla każdego e‑maila, więc ponowne użycie nazwy z innym e‑mailem kończy się niepowodzeniem. |
| E‑mail komentującego | Nie | Użytkownik jest tworzony dla tego e‑maila, jeśli jeszcze nie istnieje. |
| ID użytkownika | Nie | Istniejące ID użytkownika SSO. Ma pierwszeństwo przed nazwą i e‑mailem. |
| ID komentarza nadrzędnego | Nie | Ustaw, aby opublikować odpowiedź. |
| Zatwierdzony, Zweryfikowany | Nie | Oba domyślnie ustawione na true. Niezatwierdzone komentarze pozostają ukryte aż do moderacji. |
| Data publikacji | Nie | Domyślnie teraz. |
| URL awatara, Tytuł strony, Locale | Nie | Locale domyślnie `en_us`. |
| Pokaż na żywo w widżecie | Nie | Wysyła komentarz do widzów w czasie rzeczywistym. Kosztuje 2 kredyty zamiast 1. |
| Uruchom sprawdzanie spamu, Wyślij e‑maile | Nie | Domyślnie wyłączone. |

## Utwórz stronę

Tworzy rekord strony przed pojawieniem się na niej jakiegokolwiek komentarza, aby mogła być wyświetlana i ograniczana. Przyjmuje ID URL,
tytuł, URL oraz opcjonalnie identyfikatory grup SSO, które mają do niej dostęp.

## Utwórz użytkownika SSO

Tworzy użytkownika single sign‑on. Przyjmuje własny identyfikator użytkownika, nazwę użytkownika i e‑mail, a także opcjonalnie wyświetlaną nazwę,
etykietę wyświetlaną, awatar, stronę internetową, identyfikatory grup oraz flagi powiadomień i prywatności. Role administracyjne nie mogą
być przyznane z Zapier.

## Utwórz post w kanale

Tworzy post w kanale FastComments z treści HTML, z opcjonalnym tytułem, autorem, tagami i jedną podglądem linku.

## Utwórz hashtag

Tworzy hashtag, którego mogą używać komentujący, z opcjonalnym URL, do którego prowadzi.

## Oznacz komentarz

Oznacza komentarz do przeglądu przez moderatora. Podaj identyfikator użytkownika dokonującego oznaczenia lub pozostaw puste, aby
oznaczyć jako integracja Zapier.

## Wyszukiwania

| Wyszukiwanie | Wejście | Zwraca |
|--------------|---------|--------|
| Znajdź komentarz | ID komentarza | Komentarz lub nic. |
| Znajdź użytkownika SSO | E‑mail | Użytkownik SSO lub nic. |
| Znajdź stronę | ID URL | Strona lub nic. |

Wyszukiwanie, które nie znajduje nic, nie powoduje niepowodzenia Zapa. Połącz wyszukiwanie z tworzeniem w trybie „znajdź lub utwórz” Zapiera,
aby utworzyć stronę lub użytkownika, gdy go brakuje.