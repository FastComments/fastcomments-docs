[related-parameter-start name = 'absoluteAndRelativeDates'; type = 'boolean'; related-parameter-end]

Po podrazumevanim postavkama koriste se lokalizovani relativni datumi. Na primjer, pored nedavno ostavljenog komentara možete vidjeti "pre 11 minuta".

Može biti potrebno ili poželjno zadržati ovaj relativni format datuma, ali i prikazati puni datum pored njega — u tom slučaju postavite ovaj parametar na true. 

[code-example-start config = {absoluteAndRelativeDates: true}; linesToHighlight = [6]; title = 'Use Both Absolute and Relative Dates'; code-example-end]

Ovo se može prilagoditi bez koda, na stranici za prilagodbu widgeta, u okviru Naprednih opcija. Najprije ćete morati omogućiti Apsolutne datume da biste vidjeli ovu opciju u korisničkom interfejsu.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.absolute-dates', '.relative-and-absolute-dates']; selector = '.relative-and-absolute-dates'; title='Use Both Absolute and Relative Dates' app-screenshot-end]