[related-parameter-start name = 'voteStyle'; type = 'number'; related-parameter-end]

Standaard zal FastComments stemopties weergeven als omhoog- en omlaag-pijlen, waarmee gebruikers een opmerking omhoog of omlaag kunnen stemmen.

Het is echter mogelijk om de stijl van de stemwerkbalk te veranderen. De huidige opties zijn de standaard omhoog/omlaag-knoppen, of het gebruik van een hartstijl stemsysteem.

We gebruiken de vlag **voteStyle** als volgt:

[code-example-start config = {voteStyle: 1}; linesToHighlight = [6]; title = 'Enable Heart Button'; code-example-end]

We raden ten zeerste aan dit zonder code te doen, omdat dit ook server-side validaties inschakelt. Zie op de pagina voor widget-aanpassing de sectie "Stemstijl".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.vote-style'; selector = '.vote-style'; title='Change Voting Style' app-screenshot-end]

Stemmen kan ook worden uitgeschakeld, zie `Disable Voting` boven de stijlopties.

---