[related-parameter-start name = 'hasDarkBackground'; type = 'boolean'; related-parameter-end]

Podrazumevano, FastComments widget za komentare će automatski detektovati tamni režim na većini sajtova.

Kada se otkrije tamni režim, FastComments će promeniti prikaz sa crnog teksta na belim pozadinama na beli tekst na crnoj pozadini. Slike će se takođe promeniti.

Prilikom učitavanja stranice, widget će pokušati da odredi koliko je tamna pozadina stranice iza widgeta za komentare. Ovo znači da
stranica može imati belu pozadinu, ali ako postavite widget za komentare unutar kontejnera sa crnom pozadinom, tamni režim bi
i dalje trebalo automatski da bude omogućen kako bi komentari bili čitljivi.

Međutim, mehanizam detekcije, koji se oslanja na određivanje "luminancije", možda neće omogućiti tamni režim kada to želite. Da biste ga prisilno omogućili, postavite
*zastavicu* *hasDarkBackground* na true na sledeći način:

[code-example-start config = {hasDarkBackground: true}; linesToHighlight = [6]; title = 'Force Dark Background Mode'; additionalDemoCode = '<style>body { background: black; }</style>'; code-example-end]

---