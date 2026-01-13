[related-parameter-start name = 'defaultSortDirection'; type = 'string'; related-parameter-end]

Po defaultu, FastComments će sortirati komentare prema opciji sortiranja "Najrelevantnije".

Sortiranje "Najrelevantnije" uzima u obzir vrijeme kada je komentar ostavljen i broj glasova pri sortiranju.

Korisnik potom može promijeniti smjer sortiranja na "Najstarije prvo" ili "Najnovije prvo" u komentarskom widgetu UI.

Međutim, zadanu vrijednost možemo promijeniti na bilo koju od ove tri. Na primjer, ako želite prikazati najstarije komentare prvo:

[code-example-start config = {defaultSortDirection: "OF"}; linesToHighlight = [6]; title = 'Changing The Default Sort To Oldest First'; code-example-end]

Postavili smo vrijednost **defaultSortDirection** na "OF" da postavimo smjer na "OF".

Za smjer sortiranja "Najnovije prvo", uradili bismo sljedeće:

[code-example-start config = {defaultSortDirection: "NF"}; linesToHighlight = [6]; title = 'Changing The Default Sort To Newest First'; code-example-end]

Dozvoljene vrijednosti za **defaultSortDirection** su:

- MR: "Najnovije"
- NF: "Najnovije prvo"
- OF: "Najstarije prvo"

Ovo se može uraditi i bez koda. Na stranici za prilagođavanje widgeta, pogledajte sekciju "Podrazumijevani smjer sortiranja".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.default-sort-direction'; title='Changing The Default Sort Direction' app-screenshot-end]

Imajte na umu da su komentari na svakoj stranici za svaki smjer sortiranja prethodno izračunati, tako da sve opcije sortiranja imaju iste performanse.