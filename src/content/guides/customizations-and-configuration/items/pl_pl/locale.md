[related-parameter-start name = 'locale'; type = 'string'; related-parameter-end]

Domyślnie FastComments wyświetla widżet komentarzy w lokalizacji (locale) określonej przez system i przeglądarkę użytkownika.

Gdy użytkownik doda komentarz lub się zaloguje, aktualizujemy jego ostatnio używaną lokalizację i wykorzystujemy ją również do wysyłania e‑maili.

Ma to wpływ na sposób, w jaki widżet komentarzy jest tłumaczony dla użytkownika. Locale składa się z języka i regionu użytkownika, więc skonfigurowanie locale\nzazwyczaj zmieni język używany do wyświetlania tekstu.

#### Via The UI

Można to zdefiniować za pomocą interfejsu dostosowywania widżetu. Zobacz opcję "Locale / Language":

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.locale-override'; title='Changing The Locale / Language' app-screenshot-end]

#### Via Code

Można to nadpisać, ustawiając żądane locale.

[code-example-start config = {locale: 'ru_ru'}; linesToHighlight = [6]; title = 'Manually Defining the User\'s Locale'; code-example-end]

### Supported Languages and Locale Codes

[Pełną listę obsługiwanych języków i odpowiadających im kodów locale znajdziesz tutaj.](/guide-supported-languages.html#supported-languages)

### SSO Note

Jeśli używasz SSO, warto przekazać locale użytkownika w obiekcie użytkownika, aby e‑maile i inne treści były dla użytkownika poprawnie zlokalizowane.

---