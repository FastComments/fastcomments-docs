[related-parameter-start name = 'voteStyle'; type = 'number'; related-parameter-end]

Prema zadanim postavkama, FastComments će prikazati opcije glasanja kao strelice za gore i dolje, omogućujući korisnicima da glasaju gore ili dolje za komentar.

Međutim, moguće je promijeniti stil alatne trake za glasanje. Trenutne opcije su zadane tipke Gore/Dolje ili korištenje mehanizma glasanja u obliku srca.

Koristimo zastavicu **voteStyle** na sljedeći način:

[code-example-start config = {voteStyle: 1}; linesToHighlight = [6]; title = 'Enable Heart Button'; code-example-end]

Preporučamo da to učinite bez koda jer to također omogućuje provjere na strani poslužitelja. Na stranici za prilagodbu widgeta, pogledajte odjeljak "Vote Style".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.vote-style'; selector = '.vote-style'; alt='Postavka stila glasanja na stranici za prilagodbu widgeta, nudi strelice za gore i dolje ili glasanje srcem'; title='Promijeni stil glasanja' app-screenshot-end]

Glasanje se također može onemogućiti, pogledajte `Disable Voting` iznad opcija stila.