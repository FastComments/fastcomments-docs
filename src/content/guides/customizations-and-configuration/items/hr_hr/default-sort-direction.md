[related-parameter-start name = 'defaultSortDirection'; type = 'string'; related-parameter-end]

Prema zadanim postavkama, FastComments će sortirati komentare prema smjeru sortiranja "Najrelevantnije".

Sortiranje Najrelevantnije uzima u obzir vrijeme kada je komentar ostavljen i broj glasova za sortiranje.

Korisnik tada može promijeniti smjer sortiranja na Najstarije ili Najnovije prvo u korisničkom sučelju widgeta za komentare.

Međutim, zadanu vrijednost možemo promijeniti na bilo koju od tri. Na primjer, ako želite prikazati najstarije komentare prvi:

[code-example-start config = {defaultSortDirection: "OF"}; linesToHighlight = [6]; title = 'Promjena zadane vrste sortiranja na najstarije prvo'; code-example-end]

Postavili smo vrijednost **defaultSortDirection** na "OF" kako bismo postavili smjer na "OF".

Za smjer sortiranja najnovije prvo, učinili bismo sljedeće:

[code-example-start config = {defaultSortDirection: "NF"}; linesToHighlight = [6]; title = 'Promjena zadane vrste sortiranja na najnovije prvo'; code-example-end]

Valjane vrijednosti za **defaultSortDirection** su:

- MR: "Najnovije"
- NF: "Najnovije prvo"
- OF: "Najstarije prvo"

Ovo se također može učiniti bez koda. Na stranici za prilagodbu widgeta, pogledajte odjeljak "Default Sort Direction".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.default-sort-direction'; alt='Odabir Default Sort Direction koji nudi Najrelevantnije, Najnovije prvo i Najstarije prvo'; title='Promjena zadane vrste sortiranja' app-screenshot-end]

Napomena: komentari na svakoj stranici za svaki smjer sortiranja su unaprijed izračunati, pa svi smjerovi sortiranja imaju istu izvedbu.