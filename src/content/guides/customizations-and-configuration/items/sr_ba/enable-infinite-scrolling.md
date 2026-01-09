[related-parameter-start name = 'enableInfiniteScrolling'; type = 'boolean'; related-parameter-end]

Po zadanim postavkama, FastComments widget će se vertikalno prilagoditi kako bi smjestio sve vidljive komentare. Paginacija se postiže putem dugmeta "Prikaži sljedeće" na kraju trenutne stranice, jer smo utvrdili da je to interakcija koja većini korisnika najviše odgovara.

Međutim, postoje slučajevi gdje je poželjnije beskonačno skrolanje. Na primjer, ovu funkcionalnost koristimo u našem Stream Chat proizvodu.

Možemo sakriti dugmad "Prikaži sljedeće" i prebaciti se na beskonačno skrolanje postavljanjem zastavice **enableInfiniteScrolling** na true:

[code-example-start config = {enableInfiniteScrolling: true}; linesToHighlight = [6]; title = 'Enabling Infinite Scrolling'; code-example-end]

Za ovo je također potrebno dodati prilagođeni CSS. Dodajte prilagođeni CSS za selektor `.comments` da omogućite skrolanje, na primjer:

[inline-code-attrs-start title = 'Omogući beskonačno skrolanje'; type = 'css'; inline-code-attrs-end]
[inline-code-start]
.comments {
    max-height: 500px;
    overflow-y: auto;
}
[inline-code-end]

Kompletan radni primjer bio bi:

[code-example-start config = {enableInfiniteScrolling: true, customCSS: '.comments { max-height: 500px;  overflow-y: auto; }'}; linesToHighlight = [6, 7]; title = 'Enabling Infinite Scrolling'; code-example-end]

U gornjem primjeru koristimo svojstvo `customCSS`, međutim preporučuje se korištenje sučelja za konfiguraciju widgeta zbog razloga performansi. [Pogledajte dokumentaciju za prilagođeni CSS.](/guide-customizations-and-configuration.html#custom-css)