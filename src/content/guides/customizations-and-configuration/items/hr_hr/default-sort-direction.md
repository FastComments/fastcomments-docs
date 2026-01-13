[related-parameter-start name = 'defaultSortDirection'; type = 'string'; related-parameter-end]

Po zadanom, FastComments će sortirati komentare prema smjeru sortiranja "Najrelevantnije".

Sortiranje "Najrelevantnije" uzima u obzir vrijeme ostavljanja komentara i broj glasova prilikom sortiranja.

Korisnik zatim može promijeniti smjer sortiranja u korisničkom sučelju widgeta komentara na "Najstariji prvo" ili "Najnoviji prvo".

Međutim, zadanu vrijednost možemo postaviti na bilo koje od ta tri. Na primjer, ako želite prikazati najstarije komentare prvo:

[code-example-start config = {defaultSortDirection: "OF"}; linesToHighlight = [6]; title = 'Changing The Default Sort To Oldest First'; code-example-end]

Postavljamo vrijednost **defaultSortDirection** na "OF" kako bismo postavili smjer na "OF".

Za smjer sortiranja "Najnoviji prvo", učinili bismo sljedeće:

[code-example-start config = {defaultSortDirection: "NF"}; linesToHighlight = [6]; title = 'Changing The Default Sort To Newest First'; code-example-end]

Valjane vrijednosti za **defaultSortDirection** su:

- MR: "Most Recent"
- NF: "Newest First"
- OF: "Oldest First"

Ovo se također može učiniti bez koda. Na stranici za prilagodbu widgeta, pogledajte odjeljak "Zadani smjer sortiranja".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.default-sort-direction'; title='Changing The Default Sort Direction' app-screenshot-end]

Imajte na umu da su komentari na svakoj stranici za svaki smjer sortiranja unaprijed izračunati, tako da svi smjerovi sortiranja imaju iste performanse.