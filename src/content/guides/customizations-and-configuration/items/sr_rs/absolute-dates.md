[related-parameter-start name = 'absoluteDates'; type = 'boolean'; related-parameter-end]

Podrazumevano se koriste lokalizovani relativni datumi. Na primer, pored nedavno ostavljenog komentara možete videti "11 minutes ago".

Može biti potrebno ili poželjno koristiti apsolutne datume, u kom slučaju postavite ovaj parametar na true. 

[code-example-start config = {absoluteDates: true}; linesToHighlight = [6]; title = 'Use Absolute Dates'; code-example-end]

Ovo se može prilagoditi bez koda, na stranici za prilagođavanje widgeta, pod Naprednim opcijama:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.absolute-dates']; selector = '.absolute-dates'; alt='Napredne opcije na stranici za prilagođavanje widgeta sa uključenim prekidačem za apsolutne datume'; title='Koristi apsolutne datume' app-screenshot-end]