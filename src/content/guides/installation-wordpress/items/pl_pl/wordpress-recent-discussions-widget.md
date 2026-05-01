Widżet Najnowsze dyskusje wyświetla strony na Twojej witrynie z najnowszą aktywnością komentarzy. Jest przydatny do wyróżniania wątków, do których wciąż dodawane są komentarze, dzięki czemu odwiedzający mogą wrócić do aktywnych rozmów zamiast trafiać na ciche strony.

## Opcje

- **Tytuł** (opcjonalnie): Nagłówek wyświetlany nad listą. Domyślnie "Najnowsze dyskusje".
- **Liczba** (opcjonalnie): Ile dyskusji ma być wyświetlonych. Zakres od 1 do 50. Domyślnie 20.

## Jak dodać

### W poście lub na stronie

W edytorze bloków dodaj blok **Shortcode** i wklej:

[inline-code-attrs-start title = 'Skrót Najnowszych dyskusji'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
[fastcomments_recent_discussions count="20"]
[inline-code-end]

Atrybut `count` przyjmuje dowolną wartość z przedziału od `1` do `50`.

### W pasku bocznym lub stopce (motywy klasyczne)

Przejdź do **Wygląd > Widgety** w panelu administracyjnym WordPress. Z narzędzia do wstawiania bloków wyszukaj "FastComments" i wybierz **FastComments: Recent Discussions**. Przeciągnij go do obszaru paska bocznego, nagłówka lub stopki, a następnie skonfiguruj tytuł i liczbę w panelu widgetu.

### W motywie blokowym (edycja całej witryny)

Otwórz **Edytor witryny** w **Wygląd > Edytor**. Przejdź do części szablonu, w której ma się pojawić widżet, wstaw blok **Legacy Widget** i wybierz z rozwijanego menu **FastComments: Recent Discussions**.

## Rozwiązywanie problemów

Widżet renderuje się dopiero po zakończeniu konfiguracji FastComments i zapisaniu tenant ID. Jeśli obszar widżetu jest pusty, dokończ konfigurację w sekcji **FastComments** w panelu administracyjnym WordPress i odśwież stronę.

Jeśli kolejność dyskusji wygląda na nieaktualną, sprawdź, czy strony źródłowe zakończyły synchronizację w panelu FastComments. Widżet odczytuje dane na żywo, więc świeżo zaimportowane komentarze mogą chwilę zająć, zanim się pojawią.