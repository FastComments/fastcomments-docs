[related-parameter-start name = 'voteStyle'; type = 'number'; related-parameter-end]

Po zadanim postavkama, FastComments će prikazati opcije glasanja kao strelice gore i dolje, omogućujući korisnicima da ili glasaju za ili protiv komentara.

Međutim, moguće je promijeniti stil trake za glasanje. Trenutne opcije su zadane tipke Gore/Dolje ili korištenje mehanizma glasanja u obliku srca.

Koristimo zastavicu **voteStyle** na sljedeći način:

[code-example-start config = {voteStyle: 1}; linesToHighlight = [6]; title = 'Enable Heart Button'; code-example-end]

Toplo preporučamo da to učinite bez koda jer to također omogućuje provjere na strani poslužitelja. Na stranici za prilagodbu widgeta pogledajte odjeljak "Stil glasanja".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.vote-style'; selector = '.vote-style'; title='Change Voting Style' app-screenshot-end]

Glasanje se također može onemogućiti, pogledajte `Disable Voting` iznad opcija stila.

---