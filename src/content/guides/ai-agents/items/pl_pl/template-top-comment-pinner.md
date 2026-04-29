**ID szablonu:** `top_comment_pinner`

Top Comment Pinner obserwuje komentarze najwyższego poziomu, które przekraczają próg głosów, i przypina je - zastępując to, co było wcześniej przypięte w tym samym wątku.

Wbudowany prompt instruuje agenta, aby pomijał odpowiedzi (przypinanie działa na wątkach, więc przypinanie odpowiedzi rzadko bywa użyteczne) oraz aby filtrował treści promocyjne (tak, by agent nie zwiększał zasięgu popularnego link-spamu).

### Wyzwalacze

- **A comment crosses a vote threshold** (`COMMENT_VOTE_THRESHOLD`, default vote threshold: 10).

Wyzwalacz uruchamia się, gdy netto głosów komentarza (`up - down`) osiąga skonfigurowany próg. Dostosuj tę liczbę w formularzu edycji w zależności od aktywności Twoich wątków - 10 to rozsądna wartość domyślna dla umiarkowanie aktywnych serwisów.

### Dozwolone narzędzia

- [`pin_comment`](#tools-overview)
- [`unpin_comment`](#tools-overview)

Przypinanie jest nieinwazyjne - można je odwrócić natychmiast - dlatego ten szablon zwykle działa bez zatwierdzeń.

### Zalecane dodatki przed uruchomieniem

- **Zaznacz "Include parent comment and prior replies in the same thread"** w [Context Options](#context-options). Bez kontekstu wątku agent nie jest w stanie wiarygodnie stwierdzić, czy istnieje już przypięty komentarz do odpięcia.
- **Dostosuj próg głosów** do Twojego serwisu. Na ruchliwych wątkach 10 zdarza się zbyt często; w cichych wątkach 10 może nigdy nie wystąpić.
- **Rozważ ograniczenie zakresu według URL** jeśli chcesz przypinać komentarze tylko w określonych sekcjach serwisu - np. w wątkach wiadomości, ale nie w wątkach ogłoszeń.

### Uwaga dotycząca zdublowanego przypinania

Prompt agenta instruuje go, aby najpierw odpiął, a dopiero potem przypiął, ale jeśli model pominie ten krok, sama platforma nie wymusza zasady jednego przypięcia na wątek (możesz mieć wiele). Jeśli zdublowane przypinanie jest problemem w Twoim serwisie, zabezpiecz `pin_comment` za zgodą i przeglądaj każde przypięcie - albo napisz bardziej restrykcyjny prompt.