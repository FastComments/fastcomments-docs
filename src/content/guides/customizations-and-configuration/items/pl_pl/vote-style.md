[related-parameter-start name = 'voteStyle'; type = 'number'; related-parameter-end]

Domyślnie FastComments wyświetla opcje głosowania jako strzałki w górę i w dół, umożliwiając użytkownikom przyznanie głosu pozytywnego lub negatywnego do komentarza.

Jednakże możliwe jest zmienienie stylu paska głosowania. Obecne opcje to domyślne przyciski Góra/Dół lub użycie mechanizmu głosowania w stylu serca.

Używamy flagi **voteStyle** w następujący sposób:

[code-example-start config = {voteStyle: 1}; linesToHighlight = [6]; title = 'Włącz przycisk serca'; code-example-end]

Zdecydowanie zalecamy wykonanie tego bez kodu, ponieważ włącza to również walidacje po stronie serwera. Na stronie dostosowywania widgetu zobacz sekcję „Vote Style”.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.vote-style'; selector = '.vote-style'; alt='Ustawienie stylu głosowania na stronie dostosowywania widgetu, oferujące strzałki w górę i w dół lub głosowanie sercem'; title='Zmień styl głosowania' app-screenshot-end]

Głosowanie można również wyłączyć, zobacz `Disable Voting` powyżej opcji stylu.