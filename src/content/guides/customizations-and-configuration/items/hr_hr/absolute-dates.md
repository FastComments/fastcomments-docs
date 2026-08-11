[related-parameter-start name = 'absoluteDates'; type = 'boolean'; related-parameter-end]

Prema zadanim postavkama, koriste se lokalizirani relativni datumi. Na primjer, uz nedavno ostavljen komentar možete vidjeti "11 minuta prije".

Možda će biti potrebno ili poželjno koristiti apsolutne datume, u tom slučaju postavite ovaj parametar na true. 

[code-example-start config = {absoluteDates: true}; linesToHighlight = [6]; title = 'Koristi apsolutne datume'; code-example-end]

Ovo se može prilagoditi bez koda, na stranici prilagodbe widgeta, pod naprednim opcijama:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.absolute-dates']; selector = '.absolute-dates'; alt='Napredne opcije na stranici prilagodbe widgeta s uključenim prekidačem apsolutnih datuma'; title='Koristi apsolutne datume' app-screenshot-end]