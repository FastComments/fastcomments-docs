Widżet Top Pages wyświetla najczęściej komentowane strony w Twojej witrynie. Przydaje się do wyeksponowania najbardziej angażujących treści dla nowych odwiedzających i zwiększenia czasu spędzanego na stronie.

## Opcje

- **Title** (opcjonalnie): Nagłówek wyświetlany nad listą. Domyślnie "Top Pages".

Widżet Top Pages sam wybiera układ w oparciu o dostępne dane i nie przyjmuje atrybutu count.

## Jak dodać

### Wewnątrz wpisu lub strony

W edytorze bloków dodaj blok **Shortcode** i wklej:

[inline-code-attrs-start title = 'Skrót Top Pages'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
[fastcomments_top_pages]
[inline-code-end]

### W pasku bocznym lub stopce (motywy klasyczne)

Przejdź do **Wygląd > Widżety** w panelu administracyjnym WordPress. Z poziomu wstawiacza bloków wyszukaj "FastComments" i wybierz **FastComments: Top Pages**. Przeciągnij go do paska bocznego, nagłówka lub stopki, a następnie ustaw tytuł w panelu widżetu.

### W motywie blokowym (Pełna edycja witryny)

Otwórz **Edytor witryny** w **Wygląd > Edytor**. Przejdź do części szablonu, w której ma się pojawić widżet, wstaw blok **Legacy Widget** i wybierz **FastComments: Top Pages** z rozwijanego menu.

## Rozwiązywanie problemów

Widżet renderuje się dopiero po ukończeniu konfiguracji FastComments i zapisaniu tenant ID. Jeśli obszar widżetu jest pusty, dokończ konfigurację w sekcji **FastComments** w panelu administracyjnym WordPress i odśwież stronę.