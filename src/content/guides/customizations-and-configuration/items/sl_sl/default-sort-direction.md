[related-parameter-start name = 'defaultSortDirection'; type = 'string'; related-parameter-end]

Privzeto bo FastComments razvrstil komentarje po smeri razvrščanja "Najbolj relevantno".

Razvrščanje Najbolj relevantno upošteva čas, ko je bil komentar objavljen, in število glasov pri razvrščanju.

Uporabnik lahko nato spremeni smer razvrščanja na najstarejše ali najnovejše najprej v uporabniškem vmesniku pripomočka za komentarje.

Vendar lahko privzeto spremenimo v katerokoli od treh možnosti. Na primer, če želite prikazati najstarejše komentarje najprej:

[code-example-start config = {defaultSortDirection: "OF"}; linesToHighlight = [6]; title = 'Sprememba privzete smeri razvrščanja na najstarejše najprej'; code-example-end]

Nastavimo vrednost **defaultSortDirection** na "OF", da nastavimo smer na "OF".

Za smer razvrščanja najnovejše najprej bi naredili naslednje:

[code-example-start config = {defaultSortDirection: "NF"}; linesToHighlight = [6]; title = 'Sprememba privzete smeri razvrščanja na najnovejše najprej'; code-example-end]

Veljavne vrednosti za **defaultSortDirection** so:

- MR: "Najbolj nedavno"
- NF: "Najnovejše najprej"
- OF: "Najstarejše najprej"

To je mogoče storiti tudi brez kode. Na strani za prilagajanje pripomočka si oglejte razdelek "Privzeta smer razvrščanja".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.default-sort-direction'; alt='Izbirnik privzete smeri razvrščanja, ki ponuja Najbolj relevantno, Najnovejše najprej in Najstarejše najprej'; title='Sprememba privzete smeri razvrščanja' app-screenshot-end]

Upoštevajte, da so komentarji na vsaki strani za vsako smer razvrščanja vnaprej izračunani, zato imajo vse smeri razvrščanja enako zmogljivost.

---