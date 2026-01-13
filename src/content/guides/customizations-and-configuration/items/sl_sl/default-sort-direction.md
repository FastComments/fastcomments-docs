[related-parameter-start name = 'defaultSortDirection'; type = 'string'; related-parameter-end]

Privzeto bo FastComments razvrščal komentarje po smeri razvrščanja "Most Relevant".

Razvrščanje "Most Relevant" upošteva čas, ko je bil komentar objavljen, in število glasov pri razvrščanju.

Uporabnik lahko nato v uporabniškem vmesniku pripomočka za komentarje spremeni smer razvrščanja na bodisi "Oldest" ali "Newest First".

Vendar lahko privzeto nastavitev spremenimo na katero koli od teh treh. Na primer, če želite najprej prikazati najstarejše komentarje:

[code-example-start config = {defaultSortDirection: "OF"}; linesToHighlight = [6]; title = 'Changing The Default Sort To Oldest First'; code-example-end]

Vrednost **defaultSortDirection** nastavimo na "OF", da nastavite smer na "OF".

Za smer razvrščanja "Newest First" bi naredili naslednje:

[code-example-start config = {defaultSortDirection: "NF"}; linesToHighlight = [6]; title = 'Changing The Default Sort To Newest First'; code-example-end]

Veljavne vrednosti za **defaultSortDirection** so:

- MR: "Most Recent"
- NF: "Newest First"
- OF: "Oldest First"

To je mogoče narediti tudi brez kode. Na strani za prilagajanje pripomočka si oglejte razdelek "Default Sort Direction".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.default-sort-direction'; title='Changing The Default Sort Direction' app-screenshot-end]

Upoštevajte, da so komentarji na vsaki strani za vsako smer razvrščanja vnaprej izračunani, zato vse smeri razvrščanja nudijo enako zmogljivost.