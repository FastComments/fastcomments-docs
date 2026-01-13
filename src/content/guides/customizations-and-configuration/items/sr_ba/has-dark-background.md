[related-parameter-start name = 'hasDarkBackground'; type = 'boolean'; related-parameter-end]

Po zadanim postavkama, FastComments widget za komentare će automatski prepoznati tamni režim na većini sajtova.

Kada se otkrije tamni režim, FastComments će prebaciti crni tekst na bijeloj pozadini u bijeli tekst na crnoj pozadini. Slike će se takođe promijeniti.

Pri učitavanju stranice, widget će pokušati odrediti koliko je tamna pozadina stranice iza widgeta za komentare. To znači da
stranica može imati bijelu pozadinu, ali ako stavite widget za komentare unutar kontejnera s crnom pozadinom, tamni režim bi trebao
i dalje automatski biti omogućen kako bi komentari bili čitljivi.

Međutim, mehanizam detekcije, koji se oslanja na određivanje "luminancije", možda neće omogućiti tamni režim kada vi to želite. Da biste ga prisilno omogućili, postavite
*hasDarkBackground* zastavicu na true kako slijedi:

[code-example-start config = {hasDarkBackground: true}; linesToHighlight = [6]; title = 'Force Dark Background Mode'; additionalDemoCode = '<style>body { background: black; }</style>'; code-example-end]

---