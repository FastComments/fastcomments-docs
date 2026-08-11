[related-parameter-start name = 'voteStyle'; type = 'number'; related-parameter-end]

Standaard zal FastComments stemopties weergeven als omhoog- en omlaag-pijlen, waardoor gebruikers een reactie kunnen up- of downvoten.

Het is echter mogelijk om de stijl van de stemwerkbalk te wijzigen. De huidige opties zijn de standaard Omhoog/Omlaag-knoppen, of het gebruik van een hartstijl stemmechanisme.

We gebruiken de **voteStyle**-vlag als volgt:

[code-example-start config = {voteStyle: 1}; linesToHighlight = [6]; title = 'Schakel Hartknop in'; code-example-end]

We raden sterk aan dit zonder code te doen, omdat dit ook server-side validaties inschakelt. Op de widget-aanpassingspagina, zie de sectie "Stemstijl".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.vote-style'; selector = '.vote-style'; alt='Stemstijlinstelling op de widget-aanpassingspagina, met omhoog- en omlaag-pijlen of hartstemmen'; title='Wijzig stemstijl' app-screenshot-end]

Stemmen kan ook worden uitgeschakeld, zie `Disable Voting` boven de stijlopties.