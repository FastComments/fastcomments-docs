[related-parameter-start name = 'defaultSortDirection'; type = 'string'; related-parameter-end]

Po defaultu, FastComments će sortirati komentare prema pravcu sortiranja "Most Relevant".

Sortiranje "Most Relevant" uzima u obzir vreme kada je komentar ostavljen i broj glasova prilikom sortiranja.

Korisnik zatim može promeniti pravac sortiranja u UI-ju widgeta za komentare na Oldest ili Newest First.

Međutim, možemo promeniti podrazumevano na bilo koju od ove tri. Na primer, ako želite da prikažete najstarije komentare prvi:

[code-example-start config = {defaultSortDirection: "OF"}; linesToHighlight = [6]; title = 'Changing The Default Sort To Oldest First'; code-example-end]

Postavljamo vrednost **defaultSortDirection** na "OF" da bismo postavili smer na "OF".

Za sortiranje koje prikazuje najnovije prvo, uradili bismo sledeće:

[code-example-start config = {defaultSortDirection: "NF"}; linesToHighlight = [6]; title = 'Changing The Default Sort To Newest First'; code-example-end]

Važeće vrednosti za **defaultSortDirection** su:

- MR: "Most Recent"
- NF: "Newest First"
- OF: "Oldest First"

Ovo se takođe može uraditi bez koda. Na stranici za prilagođavanje widgeta, pogledajte odeljak "Default Sort Direction".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.default-sort-direction'; title='Changing The Default Sort Direction' app-screenshot-end]

Napomena: komentari na svakoj stranici za svaki pravac sortiranja su prethodno izračunati, tako da svi pravci sortiranja imaju isti nivo performansi.