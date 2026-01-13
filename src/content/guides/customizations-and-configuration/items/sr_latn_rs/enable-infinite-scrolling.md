[related-parameter-start name = 'enableInfiniteScrolling'; type = 'boolean'; related-parameter-end]

Po podrazumevanoj vrednosti, FastComments widget će se vertikalno prilagoditi tako da stane sve vidljive komentare. Paginacija se postiže putem dugmeta "Prikaži sledeće"
na kraju trenutne stranice, jer smo utvrdili da je to interakcija koja najviše odgovara većini korisnika.

Međutim, postoje slučajevi kada se preferira beskonačno skrolovanje. Na primer, ovu funkciju koristimo u našem proizvodu Stream Chat.

Možemo sakriti dugmad "Prikaži sledeće" i preći na beskonačno skrolovanje podešavanjem zastavice **enableInfiniteScrolling** na true:

[code-example-start config = {enableInfiniteScrolling: true}; linesToHighlight = [6]; title = 'Enabling Infinite Scrolling'; code-example-end]

Ovo takođe zahteva dodavanje prilagođenog CSS-a. Dodajte prilagođeni CSS za selektor `.comments` da omogućite skrolovanje, na primer:

[inline-code-attrs-start title = 'Omogućavanje beskonačnog skrolovanja'; type = 'css'; inline-code-attrs-end]
[inline-code-start]
.comments {
    max-height: 500px;
    overflow-y: auto;
}
[inline-code-end]

Kompletan radni primer bi bio:

[code-example-start config = {enableInfiniteScrolling: true, customCSS: '.comments { max-height: 500px;  overflow-y: auto; }'}; linesToHighlight = [6, 7]; title = 'Enabling Infinite Scrolling'; code-example-end]

U gornjem primeru koristimo svojstvo `customCSS`, međutim preporučuje se da se umesto toga koristi Widget Configuration UI iz razloga performansi. [Pogledajte dokumentaciju o prilagođenom CSS-u.](/guide-customizations-and-configuration.html#custom-css)