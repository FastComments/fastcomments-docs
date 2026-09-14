## Akcje i Wyszukiwania

Akcje tworzą dane w FastComments; wyszukiwania odszukują dane, aby późniejszy krok mógł je wykorzystać. Każda akcja wywołuje API REST FastComments i zużywa taką samą liczbę kredytów API, jaką kosztowałoby wywołanie z własnego kodu: jeden kredyt za wywołanie, chyba że zaznaczono inaczej.

## Utwórz komentarz

Publikuje komentarz na stronie.

| Pole | Wymagane | Uwagi |
|------|----------|-------|
| Page URL ID | Tak | Identyfikator URL, którego używa widget komentarzy na stronie. Komentarze są grupowane według niego. |
| Page URL | Tak | Pełny adres URL strony, używany w e‑mailach powiadamiających. |
| Comment | Tak | Treść komentarza w formacie markdown FastComments. |
| Commenter Name | Tak | Nazwy są unikalne dla każdego e‑maila, więc ponowne użycie nazwy z innym e‑mailem kończy się niepowodzeniem. |
| Commenter Email | Nie | Użytkownik jest tworzony dla tego e‑maila, jeśli jeszcze nie istnieje. |
| User ID | Nie | Istniejący identyfikator użytkownika SSO. Ma pierwszeństwo przed nazwą i e‑mailem. |
| Parent Comment ID | Nie | Ustaw, aby opublikować odpowiedź. |
| Approved, Verified | Nie | Oba domyślnie ustawione na true. Niezatwierdzone komentarze pozostają ukryte, dopóki nie zostaną moderowane. |
| Posted At | Nie | Domyślnie ustawione na bieżącą datę i godzinę. |
| Avatar URL, Page Title, Locale | Nie | Locale domyślnie `en_us`. |
| Show Live In Widget | Nie | Wysyła komentarz do widzów w czasie rzeczywistym. Kosztuje 2 kredyty zamiast 1. |
| Run Spam Check, Send Emails | Nie | Domyślnie wyłączone. |

## Utwórz lub zaktualizuj stronę

Tworzy rekord strony przed pojawieniem się na niej jakiegokolwiek komentarza, aby mogła być wyświetlana i ograniczana. Przyjmuje identyfikator URL, tytuł, URL oraz opcjonalnie identyfikatory grup SSO, które mają do niej dostęp. Jeśli strona o podanym identyfikatorze URL już istnieje, jest aktualizowana przy użyciu podanych pól, więc Zap może być uruchamiany wielokrotnie dla tej samej strony.

## Utwórz lub zaktualizuj użytkownika SSO

Tworzy użytkownika jednokrotnego logowania (single sign‑on). Przyjmuje własny identyfikator użytkownika, nazwę użytkownika i e‑mail, a także opcjonalnie wyświetlaną nazwę, etykietę, awatar, stronę internetową, identyfikatory grup oraz flagi powiadomień i prywatności. Jeśli użytkownik o podanym identyfikatorze już istnieje, zostaje zaktualizowany. Role administracyjne nie mogą być przyznawane z poziomu Zapier.

## Utwórz post w kanale

Tworzy post w kanale FastComments z treści HTML. Wymagany jest identyfikator użytkownika autora (identyfikator FastComments lub SSO); tytuł, tagi i podgląd jednego linku są opcjonalne.

## Utwórz lub zaktualizuj hashtag

Tworzy hashtag, którego mogą używać komentujący, z opcjonalnym URL, do którego prowadzi. Jeśli hashtag już istnieje, zostaje zaktualizowany.

## Oznacz komentarz

Oznacza komentarz do przeglądu przez moderatora. Wymagany jest identyfikator użytkownika dokonującego oznaczenia; identyfikator autora zwrócony przez Utwórz komentarz działa.

## Wyszukiwania

| Wyszukiwanie | Wejście | Zwraca |
|--------------|---------|--------|
| Find Comment | Comment ID | Komentarz lub brak wyniku. |
| Find SSO User | Email | Użytkownik SSO lub brak wyniku. |
| Find Page | URL ID | Strona lub brak wyniku. |

Wyszukiwanie, które nie znajduje nic, nie powoduje niepowodzenia Zapa. Find SSO User i Find Page oferują opcję Zapier „utwórz, jeśli nie istnieje”, która uruchamia odpowiednie tworzenie, gdy nic nie zostanie znalezione.