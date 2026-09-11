## Akcje i Wyszukiwania

Akcje tworzą dane w FastComments; wyszukiwania przeglądają dane, aby późniejszy krok mógł je wykorzystać. Każda akcja wywołuje interfejs FastComments REST API i zużywa taką samą liczbę kredytów API, jaką kosztowałoby to wywołanie w Twoim własnym kodzie: jeden kredyt za wywołanie, chyba że zaznaczono inaczej.

## Utwórz komentarz

Publikuje komentarz na stronie.

| Pole | Wymagane | Uwagi |
|------|----------|-------|
| ID URL strony | Tak | ID URL używany przez widżet komentarzy na stronie. Komentarze są grupowane według niego. |
| URL strony | Tak | Pełny URL strony, używany w e‑mailach powiadamiających. |
| Komentarz | Tak | Treść komentarza w formacie markdown FastComments. |
| Nazwa komentującego | Tak | Nazwy są unikalne dla każdego e‑maila, więc ponowne użycie nazwy z innym e‑mailem kończy się niepowodzeniem. |
| E‑mail komentującego | Nie | Użytkownik jest tworzony dla tego e‑maila, jeśli jeszcze nie istnieje. |
| ID użytkownika | Nie | Istniejące ID użytkownika SSO. Ma pierwszeństwo przed nazwą i e‑mailem. |
| ID komentarza nadrzędnego | Nie | Ustaw, aby opublikować odpowiedź. |
| Zatwierdzony, Zweryfikowany | Nie | Oba domyślnie ustawione na true. Niezatwierdzone komentarze pozostają ukryte, dopóki nie zostaną moderowane. |
| Opublikowano o | Nie | Domyślnie bieżący czas. |
| URL awatara, Tytuł strony, Locale | Nie | Locale domyślnie `en_us`. |
| Pokaż na żywo w widżecie | Nie | Wysyła komentarz do widzów w czasie rzeczywistym. Kosztuje 2 kredyty zamiast 1. |
| Uruchom sprawdzanie spamu, Wyślij e‑maile | Nie | Domyślnie wyłączone. |

## Utwórz stronę

Tworzy rekord strony przed pojawieniem się na niej jakiegokolwiek komentarza, aby mogła być wyświetlana i ograniczana. Przyjmuje ID URL, tytuł, URL oraz opcjonalnie identyfikatory grup SSO, które mają do niej dostęp.

## Utwórz użytkownika SSO

Tworzy użytkownika single sign‑on. Przyjmuje własne ID użytkownika, nazwę użytkownika i e‑mail, a także opcjonalnie wyświetlaną nazwę, etykietę wyświetlania, awatar, stronę internetową, identyfikatory grup oraz flagi powiadomień i prywatności. Role administracyjne nie mogą być przyznane z Zapiera.

## Utwórz post w kanale

Tworzy post w kanale FastComments z treści HTML. Wymagane jest ID użytkownika autora (ID użytkownika FastComments lub SSO); tytuł, tagi i podgląd jednego linku są opcjonalne.

## Utwórz hashtag

Tworzy hashtag, którego mogą używać komentujący, z opcjonalnym URL, do którego prowadzi. Tagi są unikalne w ramach konta, więc Zap, który tworzący jeden przy każdym uruchomieniu, potrzebuje czegoś unikalnego w tagu.

## Oznacz komentarz

Oznacza komentarz do przeglądu przez moderatora. Wymagane jest ID użytkownika dokonującego oznaczenia; ID autora zwrócone przez Utwórz komentarz działa.

## Wyszukiwania

| Wyszukiwanie | Wejście | Zwraca |
|--------------|---------|--------|
| Znajdź komentarz | ID komentarza | Komentarz lub nic. |
| Znajdź użytkownika SSO | E‑mail | Użytkownik SSO lub nic. |
| Znajdź stronę | ID URL | Strona lub nic. |

Wyszukiwanie, które nic nie znajdzie, nie powoduje błędu w Zapie. Połącz wyszukiwanie z tworzeniem w trybie „znajdź lub utwórz” Zapiera, aby utworzyć stronę lub użytkownika, gdy go brakuje.