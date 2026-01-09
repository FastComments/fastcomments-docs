[related-parameter-start name = 'enableInfiniteScrolling'; type = 'boolean'; related-parameter-end]

Po defaultu, FastComments widget će se vertikalno prilagoditi kako bi smjestio sve vidljive komentare. Paginacija se postiže pomoću dugmeta „Prikaži sljedeće“ na kraju tekuće stranice, jer smo utvrdili da je to interakcija koja većini korisnika najviše odgovara.

Međutim, postoje situacije u kojima je poželjno beskonačno skrolovanje. Na primjer, ovu funkciju koristimo u našem proizvodu Stream Chat.

Možemo sakriti dugmad „Prikaži sljedeće“ i prebaciti se na beskonačno skrolovanje postavljanjem zastavice **enableInfiniteScrolling** na true:

[code-example-start config = {enableInfiniteScrolling: true}; linesToHighlight = [6]; title = 'Enabling Infinite Scrolling'; code-example-end]

Ovo također zahtijeva dodavanje prilagođenog CSS-a. Dodajte prilagođeni CSS za selektor `.comments` kako biste omogućili skrolovanje, na primjer:

[inline-code-attrs-start title = 'Omogućavanje beskonačnog skrolovanja'; type = 'css'; inline-code-attrs-end]
[inline-code-start]
.comments {
    max-height: 500px;
    overflow-y: auto;
}
[inline-code-end]

Potpuni radni primjer bio bi:

[code-example-start config = {enableInfiniteScrolling: true, customCSS: '.comments { max-height: 500px;  overflow-y: auto; }'}; linesToHighlight = [6, 7]; title = 'Enabling Infinite Scrolling'; code-example-end]

U gornjem primjeru koristimo svojstvo `customCSS`, međutim preporučuje se koristiti Widget Configuration UI umjesto toga zbog razloga performansi. [Pogledajte dokumentaciju o prilagođenom CSS-u.](/guide-customizations-and-configuration.html#custom-css)