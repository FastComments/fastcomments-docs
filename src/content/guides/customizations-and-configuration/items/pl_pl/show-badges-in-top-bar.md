---
[related-parameter-start name = 'showBadgesInTopBar'; type = 'boolean'; related-parameter-end]

Domyślnie FastComments wyświetla odznaki użytkowników tylko przy ich komentarzach w wątku komentarzy.

Jednak możemy wyświetlić odznaki użytkowników obok ich imienia nad formularzem komentarza, włączając tę funkcję na stronie dostosowywania widgetu:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.show-badges-in-top-bar'; alt='Pole wyboru wyświetlania odznak w pasku górnym na stronie dostosowywania widgetu, umieszczające odznaki obok nazwy nad formularzem komentarza'; title='Opcja wyświetlania odznak w pasku górnym' app-screenshot-end]

Spowoduje to wyświetlenie odznak użytkownika obok jego imienia w obszarze paska górnego, co sprawi, że jego osiągnięcia i status będą bardziej widoczne podczas pisania komentarza.

Uwaga: ta funkcja musi być włączona w interfejsie dostosowywania widgetu, aby działała. Opcjonalnie możesz ustawić flagę **showBadgesInTopBar** na false w konfiguracji kodu, aby selektywnie wyłączyć ją, nawet gdy jest włączona na poziomie serwera:

[code-example-start config = {showBadgesInTopBar: false}; linesToHighlight = [6]; title = 'Disable Show Badges in Top Bar'; code-example-end]
---