[related-parameter-start name = 'voteStyle'; type = 'number'; related-parameter-end]

Po podrazumevanju, FastComments prikazuje opcije glasanja kao strelice gore i dole, omogućavajući korisnicima da glasaju za ili protiv komentara.

Međutim, moguće je promeniti stil alatne trake za glasanje. Trenutne opcije su podrazumevana dugmad Gore/Dole, ili korišćenje stila glasanja sa srcem.

Koristimo zastavicu **voteStyle** na sledeći način:

[code-example-start config = {voteStyle: 1}; linesToHighlight = [6]; title = 'Enable Heart Button'; code-example-end]

Toplo preporučujemo da ovo uradite bez koda, jer to takođe omogućava validacije na serverskoj strani. Na stranici za prilagođavanje widgeta pogledajte odeljak "Stil glasanja".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.vote-style'; selector = '.vote-style'; title='Change Voting Style' app-screenshot-end]

Glasanje takođe može biti onemogućeno, pogledajte `Disable Voting` iznad opcija stila.