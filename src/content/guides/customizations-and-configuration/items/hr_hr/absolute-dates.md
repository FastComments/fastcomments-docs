[related-parameter-start name = 'absoluteDates'; type = 'boolean'; related-parameter-end]

Po zadanim postavkama koriste se lokalizirani relativni datumi. Na primjer, pored nedavno ostavljenog komentara možete vidjeti "prije 11 minuta".

Može biti potrebno ili poželjno koristiti apsolutne datume, u kojem slučaju postavite ovaj parametar na true. 

[code-example-start config = {absoluteDates: true}; linesToHighlight = [6]; title = 'Use Absolute Dates'; code-example-end]

Ovo se može prilagoditi bez koda, na stranici za prilagodbu widgeta, pod Naprednim opcijama:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.absolute-dates']; selector = '.absolute-dates'; title='Use Absolute Dates' app-screenshot-end]

---