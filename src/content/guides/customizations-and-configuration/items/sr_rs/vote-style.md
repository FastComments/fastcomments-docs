[related-parameter-start name = 'voteStyle'; type = 'number'; related-parameter-end]

Podrazumevano, FastComments će prikazati opcije glasanja kao strelice za gore i dole, omogućavajući korisnicima da glasaju gore ili dole za komentar.

Međutim, moguće je promeniti stil trake za glasanje. Trenutne opcije su podrazumevane dugmiće Gore/Dole, ili korišćenje mehanizma glasanja u obliku srca.

Koristimo zastavicu **voteStyle** na sledeći način:

[code-example-start config = {voteStyle: 1}; linesToHighlight = [6]; title = 'Enable Heart Button'; code-example-end]

Preporučujemo da ovo uradite bez koda, jer tako omogućavate i server‑side validacije. Na stranici za prilagođavanje widgeta, pogledajte odeljak „Vote Style“.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.vote-style'; selector = '.vote-style'; alt='Podešavanje stila glasanja na stranici za prilagođavanje widgeta, nudi strelice za gore i dole ili glasanje srcem'; title='Promeni stil glasanja' app-screenshot-end]

Glasanje se takođe može onemogućiti, pogledajte `Disable Voting` iznad opcija za stil.