[related-parameter-start name = 'locale'; type = 'string'; related-parameter-end]

Domyślnie FastComments wyświetli widżet komentarzy w ustawieniach regionalnych określonych przez system i przeglądarkę użytkownika.

Gdy użytkownik komentuje lub loguje się, aktualizujemy jego ostatnio używane ustawienia regionalne i używamy ich również do wysyłania e‑maili.

Ma to wpływ na to, jak widżet komentarzy jest tłumaczony dla użytkownika. Ustawienia regionalne składają się z języka i regionu użytkownika, więc ich konfiguracja zazwyczaj zmieni język wyświetlanego tekstu.

#### Via The UI

Można to określić za pomocą interfejsu dostosowywania widżetu. Zobacz opcję „Ustawienia regionalne / Język”:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.locale-override'; alt='Lista rozwijana Locale / Language na stronie dostosowywania widżetu używana do nadpisania wykrytego ustawienia regionalnego odwiedzającego'; title='Zmiana ustawień regionalnych / języka' app-screenshot-end]

#### Via Code

Można to nadpisać wybranymi ustawieniami regionalnymi.

[code-example-start config = {locale: 'ru_ru'}; linesToHighlight = [6]; title = 'Ręczne definiowanie ustawień regionalnych użytkownika'; code-example-end]

### Supported Languages and Locale Codes

[Pełną listę obsługiwanych języków i odpowiadających im kodów regionalnych znajdziesz tutaj.](/guide-supported-languages.html#supported-languages)

### SSO Note

Jeśli używasz SSO, możesz chcieć przekazać ustawienia regionalne użytkownika w obiekcie użytkownika, aby e‑maile i inne elementy były prawidłowo lokalizowane dla niego.