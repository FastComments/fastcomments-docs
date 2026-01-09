[related-parameter-start name = 'hasDarkBackground'; type = 'boolean'; related-parameter-end]

Po zadanim postavkama, FastComments widget za komentare automatski će otkriti tamni način rada na većini stranica.

Kad se otkrije tamni način rada, FastComments će prebaciti crni tekst na bijelim pozadinama u bijeli tekst na crnoj pozadini. Slike će se također promijeniti.

Prilikom učitavanja stranice, widget će pokušati odrediti koliko je pozadina stranice tamna iza widgeta za komentare. To znači da
stranica može imati bijelu pozadinu, ali ako postavite widget za komentare unutar spremnika s crnom pozadinom, tamni način rada bi trebao
i dalje biti automatski omogućen kako bi komentari bili čitljivi.

Međutim, mehanizam detekcije, koji se oslanja na određivanje "luminancije", možda neće omogućiti tamni način rada kada želite. Da biste ga prisilno omogućili, postavite
*hasDarkBackground* flag na true kako slijedi:

[code-example-start config = {hasDarkBackground: true}; linesToHighlight = [6]; title = 'Force Dark Background Mode'; additionalDemoCode = '<style>body { background: black; }</style>'; code-example-end]