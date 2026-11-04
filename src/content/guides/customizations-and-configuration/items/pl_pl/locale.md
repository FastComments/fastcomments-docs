[related-parameter-start name = 'locale'; type = 'string'; related-parameter-end]

Domyślnie FastComments wyświetli widżet komentarzy w ustawieniach regionalnych (locale) określonych przez system i przeglądarkę użytkownika.

Gdy użytkownik komentuje lub się loguje, aktualizujemy jego ostatnio używane locale i używamy go również do wysyłania e-maili.

Wpływa to na to, jak widżet komentarzy jest tłumaczony dla użytkownika. Locale składa się z języka i regionu użytkownika, więc skonfigurowanie locale zazwyczaj zmieni język używany do wyświetlania tekstu użytkownikowi.

#### Za pomocą interfejsu użytkownika

Można to zdefiniować za pomocą interfejsu dostosowywania widżetu. Zobacz opcję "Locale / Language":

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.locale-override'; alt='Lista rozwijana Locale / Language na stronie dostosowywania widżetu służąca do nadpisania wykrytego locale odwiedzającego'; title='Zmiana Locale / Języka' app-screenshot-end]

#### Za pomocą kodu

Można to nadpisać żądanym locale.

[code-example-start config = {locale: 'ru_ru'}; linesToHighlight = [6]; title = 'Ręczne definiowanie locale użytkownika'; code-example-end]

### Obsługiwane języki i kody locale

[Pełną listę obsługiwanych języków oraz odpowiadających im kodów locale znajdziesz tutaj.](/guide-supported-languages.html#supported-languages)

### Uwaga dotycząca SSO

Jeśli korzystasz z SSO, możesz chcieć przekazać locale użytkownika w obiekcie użytkownika, aby e-maile i inne rzeczy były dla niego poprawnie zlokalizowane.