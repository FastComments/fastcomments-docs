[related-parameter-start name = 'showBadgesInTopBar'; type = 'boolean'; related-parameter-end]

Domyślnie FastComments wyświetla odznaki użytkowników tylko przy ich komentarzach w wątku komentarzy.

Możemy jednak wyświetlić odznaki użytkowników obok ich nazwy nad formularzem komentarza, włączając tę funkcję na stronie dostosowywania widżetu:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.show-badges-in-top-bar'; title='Show Badges in Top Bar Option' app-screenshot-end]

Spowoduje to wyświetlenie odznak użytkownika obok jego nazwy w obszarze górnego paska, dzięki czemu jego osiągnięcia i status będą bardziej widoczne podczas pisania komentarza.

Uwaga: ta funkcja musi być włączona w interfejsie dostosowywania widżetu, aby działała. Opcjonalnie możesz ustawić flagę **showBadgesInTopBar** na false w konfiguracji kodu, aby selektywnie ją wyłączyć nawet gdy jest włączona na poziomie serwera:

[code-example-start config = {showBadgesInTopBar: false}; linesToHighlight = [6]; title = 'Disable Show Badges in Top Bar'; code-example-end]
---