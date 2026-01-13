[related-parameter-start name = 'enableInfiniteScrolling'; type = 'boolean'; related-parameter-end]

Po defaultu, FastComments widget će se vertikalno promijeniti veličinu kako bi odgovarao svim vidljivim komentarima. Paginacija se ostvaruje putem gumba "View Next" na kraju trenutne stranice, budući da smo ustanovili da je to interakcija koja većini korisnika najviše odgovara.

Međutim, postoje slučajevi u kojima je poželjno beskonačno pomicanje. Na primjer, ovu značajku koristimo u našem Stream Chat proizvodu.

Možemo sakriti gumbe "View Next" i prebaciti se na beskonačno pomicanje postavljanjem zastavice **enableInfiniteScrolling** na true:

[code-example-start config = {enableInfiniteScrolling: true}; linesToHighlight = [6]; title = 'Enabling Infinite Scrolling'; code-example-end]

To također zahtijeva dodavanje prilagođenog CSS-a. Dodajte prilagođeni CSS za selektor `.comments` kako biste omogućili pomicanje, na primjer:

[inline-code-attrs-start title = 'Omogućavanje beskonačnog pomicanja'; type = 'css'; inline-code-attrs-end]
[inline-code-start]
.comments {
    max-height: 500px;
    overflow-y: auto;
}
[inline-code-end]

Potpuni radni primjer bio bi:

[code-example-start config = {enableInfiniteScrolling: true, customCSS: '.comments { max-height: 500px;  overflow-y: auto; }'}; linesToHighlight = [6, 7]; title = 'Enabling Infinite Scrolling'; code-example-end]

U gornjem primjeru koristimo svojstvo `customCSS`, međutim preporučuje se korištenje Widget Configuration UI umjesto toga iz razloga izvedbe. [Pogledajte dokumentaciju o prilagođenom CSS-u.](/guide-customizations-and-configuration.html#custom-css)

---