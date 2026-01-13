[related-parameter-start name = 'voteStyle'; type = 'number'; related-parameter-end]

Domyślnie FastComments wyświetla opcje głosowania jako strzałki w górę i w dół, umożliwiając użytkownikom oddanie głosu za lub przeciw komentarzowi.

Jednak można zmienić styl paska narzędzi głosowania. Obecne opcje to domyślne przyciski W górę/W dół lub użycie mechanizmu głosowania w stylu serca.

Używamy flagi **voteStyle** w następujący sposób:

[code-example-start config = {voteStyle: 1}; linesToHighlight = [6]; title = 'Enable Heart Button'; code-example-end]

Zdecydowanie zalecamy zrobienie tego bez kodu, ponieważ włącza to również walidacje po stronie serwera. Na stronie dostosowywania widżetu zobacz sekcję "Vote Style".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.vote-style'; selector = '.vote-style'; title='Change Voting Style' app-screenshot-end]

Głosowanie można również wyłączyć — zobacz `Disable Voting` powyżej opcji stylu.