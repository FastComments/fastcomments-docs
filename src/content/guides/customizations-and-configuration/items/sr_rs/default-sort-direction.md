[related-parameter-start name = 'defaultSortDirection'; type = 'string'; related-parameter-end]

Podrazumevano, FastComments sortira komentare prema smeru sortiranja „Najrelevantnije“.

Sortiranje po najrelevantnijem uzima u obzir vreme kada je komentar ostavljen i broj glasova za sortiranje.

Korisnik zatim može da promeni smer sortiranja na „Najstarije prvo“ ili „Najnovije prvo“ u UI‑u vidžeta za komentare.

Međutim, možemo promeniti podrazumevano na bilo koji od tri. Na primer, ako želite da prikažete najstarije komentare prvo:

[code-example-start config = {defaultSortDirection: "OF"}; linesToHighlight = [6]; title = 'Promena podrazumevanog sortiranja na najstarije prvo'; code-example-end]

Postavljamo vrednost **defaultSortDirection** na "OF" da postavimo smer na "OF".

Za smer sortiranja „Najnovije prvo“, uradićemo sledeće:

[code-example-start config = {defaultSortDirection: "NF"}; linesToHighlight = [6]; title = 'Promena podrazumevanog sortiranja na najnovije prvo'; code-example-end]

Važeće vrednosti za **defaultSortDirection** su:

- MR: "Najnovije"
- NF: "Najnovije prvo"
- OF: "Najstarije prvo"

Ovo se takođe može uraditi bez koda. Na stranici za prilagođavanje vidžeta, pogledajte odeljak „Podrazumevani smer sortiranja“.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.default-sort-direction'; alt='Selektor podrazumevanog pravca sortiranja koji nudi Najrelevantnije, Najnovije prvo i Najstarije prvo'; title='Promena podrazumevanog pravca sortiranja' app-screenshot-end]

Napomena: komentari na svakoj stranici za svaki smer sortiranja su unapred izračunati, tako da svi smerovi sortiranja imaju istu performansu.