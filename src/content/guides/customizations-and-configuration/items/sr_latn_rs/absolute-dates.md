[related-parameter-start name = 'absoluteDates'; type = 'boolean'; related-parameter-end]

Podrazumevano se koriste lokalizovani relativni datumi. Na primer, pored nedavno ostavljenog komentara možete videti "pre 11 minuta".

Može biti potrebno ili poželjno koristiti apsolutne datume, u kom slučaju podesite ovaj parametar na true. 

[code-example-start config = {absoluteDates: true}; linesToHighlight = [6]; title = 'Koristite apsolutne datume'; code-example-end]

Ovo se može prilagoditi bez koda, na stranici za prilagođavanje widgeta, pod Naprednim opcijama:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.absolute-dates']; selector = '.absolute-dates'; title='Koristite apsolutne datume' app-screenshot-end]