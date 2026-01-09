[related-parameter-start name = 'voteStyle'; type = 'number'; related-parameter-end]

Po zadanom, FastComments prikazuje opcije za glasanje kao strelice gore i dolje, omogućavajući korisnicima da komentaru daju pozitivan ili negativan glas.

Međutim, moguće je promijeniti stil trake za glasanje. Trenutne opcije su zadane tipke Gore/Dolje, ili korištenje mehanizma glasanja u obliku srca.

Koristimo zastavicu **voteStyle** na sljedeći način:

[code-example-start config = {voteStyle: 1}; linesToHighlight = [6]; title = 'Enable Heart Button'; code-example-end]

Toplo preporučujemo da to uradite bez koda, jer tako omogućavate i provjere na strani servera. Na stranici za prilagođavanje widgeta, pogledajte odjeljak "Stil glasanja".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.vote-style'; selector = '.vote-style'; title='Change Voting Style' app-screenshot-end]

Glasanje se također može onemogućiti, pogledajte `Disable Voting` iznad opcija stila.

---