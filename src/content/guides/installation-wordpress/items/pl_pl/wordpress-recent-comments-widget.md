Widżet Recent Comments wyświetla najnowsze komentarze opublikowane w całej Twojej witrynie. Jest przydatny w paskach bocznych, stopkach lub wszędzie tam, gdzie chcesz pokazać świeżą aktywność, aby zachęcić do dalszego czytania.

## Opcje

- **Title** (opcjonalnie): Nagłówek wyświetlany nad listą. Domyślnie "Recent Comments".
- **Count** (opcjonalnie): Liczba wyświetlanych komentarzy. Zakres od 1 do 50. Domyślnie 5.

## Jak go dodać

### Wewnątrz wpisu lub strony

W edytorze bloków dodaj blok **Shortcode** i wklej:

[inline-code-attrs-start title = 'Shortcode Recent Comments'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
[fastcomments_recent_comments count="5"]
[inline-code-end]

Atrybut `count` przyjmuje dowolną wartość od 1 do 50.

### W pasku bocznym lub stopce (motywy klasyczne)

Przejdź do **Wygląd > Widgety** w panelu administracyjnym WordPress. W inserterze bloków wyszukaj "FastComments" i wybierz **FastComments: Recent Comments**. Przeciągnij go do obszaru paska bocznego, nagłówka lub stopki, a następnie skonfiguruj tytuł i liczbę z panelu widżetu.

### W motywie blokowym (pełna edycja witryny)

Otwórz **Edytor witryny** w **Wygląd > Edytor**. Przejdź do części szablonu, w której ma pojawić się widżet, wstaw blok **Legacy Widget** i wybierz **FastComments: Recent Comments** z listy rozwijanej.

## Rozwiązywanie problemów

Widżet renderuje się dopiero po zakończeniu konfiguracji FastComments i zapisaniu identyfikatora tenanta. Jeśli obszar widżetu jest pusty, dokończ konfigurację w sekcji **FastComments** w panelu administracyjnym WordPress i odśwież stronę.