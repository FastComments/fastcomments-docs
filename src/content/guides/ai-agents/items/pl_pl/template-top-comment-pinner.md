**Template ID:** `top_comment_pinner`

Top Comment Pinner monitoruje komentarze najwyższego poziomu, które przekraczają próg głosów, i przypina je - zastępując to, co było wcześniej przypięte w tym samym wątku.

### Wbudowany początkowy prompt

[inline-code-attrs-start title = 'Początkowy prompt szablonu Top Comment Pinner'; type='text' inline-code-attrs-end]
[inline-code-start]
Przypinasz najlepsze komentarze najwyższego poziomu w wątku. Gdy komentarz osiągnie próg głosów, przypnij go, jeśli treść jest merytoryczna i nie ma charakteru promocyjnego. Najpierw odepnij wszelkie wcześniej przypięte komentarze w tym samym wątku. Nie przypinaj odpowiedzi, tylko komentarze najwyższego poziomu.
[inline-code-end]

Instrukcja "nie przypinaj odpowiedzi" ma znaczenie: przypinanie działa na wątki, więc przypinanie odpowiedzi rzadko bywa użyteczne. Filtr "bez charakteru promocyjnego" zapobiega wzmacnianiu przez agenta popularnego komentarza z link-spamem.

### Wyzwalacze

- **Komentarz przekracza próg głosów** (`COMMENT_VOTE_THRESHOLD`, domyślny próg: 10).

Wyzwalacz uruchamia się, gdy wynik głosów komentarza (`up - down`) osiągnie skonfigurowany próg. Dopasuj tę liczbę w formularzu edycji w zależności od aktywności wątków - 10 to rozsądna wartość domyślna dla umiarkowanie aktywnych serwisów.

### Dozwolone narzędzia

- [`pin_comment`](#tools-overview)
- [`unpin_comment`](#tools-overview)

Przypinanie nie jest destrukcyjne - można je natychmiast cofnąć - więc ten szablon zazwyczaj działa bez zatwierdzeń.

### Zalecane dodatki przed uruchomieniem

- **Zaznacz "Uwzględnij komentarz nadrzędny i wcześniejsze odpowiedzi w tym samym wątku"** w [Context Options](#context-options). Bez kontekstu wątku agent nie może wiarygodnie stwierdzić, czy istnieje już przypięty komentarz do odpięcia.
- **Dopasuj próg głosów** do swojego serwisu. Na ruchliwych wątkach 10 zdarza się zbyt często; w cichych wątkach 10 może nigdy nie wystąpić.
- **Rozważ ograniczenie zakresu według adresu URL**, jeśli chcesz przypinać komentarze tylko w niektórych sekcjach serwisu - na przykład w wątkach z wiadomościami, ale nie w wątkach ogłoszeń.

### Uwaga dotycząca podwójnego przypinania

Prompt agenta instruuje go, aby najpierw odpiął przed przypięciem, ale jeśli model pominie ten krok, sama platforma nie wymusza reguły jednego przypiętego komentarza na wątek (możesz mieć ich kilka). Jeśli podwójne przypinanie jest problemem na Twojej stronie, umieść `pin_comment` za mechanizmem zatwierdzania i przeglądaj każde przypięcie - albo napisz dokładniejszy prompt.

---