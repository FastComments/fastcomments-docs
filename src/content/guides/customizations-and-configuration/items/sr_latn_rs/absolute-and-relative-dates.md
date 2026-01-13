[related-parameter-start name = 'absoluteAndRelativeDates'; type = 'boolean'; related-parameter-end]

Podrazumevano se koriste lokalizovani relativni datumi. Na primer, pored komentara koji je nedavno ostavljen možete videti "pre 11 minuta".

Može biti potrebno ili poželjno zadržati ovaj relativni format datuma, ali takođe prikazati i puni datum pored njega. U tom slučaju postavite ovaj parametar na true.

[code-example-start config = {absoluteAndRelativeDates: true}; linesToHighlight = [6]; title = 'Use Both Absolute and Relative Dates'; code-example-end]

Ovo se može prilagoditi bez koda, na stranici za prilagođavanje widgeta, pod Naprednim opcijama. Najpre ćete morati omogućiti Apsolutne datume da biste ovu opciju videli u korisničkom interfejsu.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.absolute-dates', '.relative-and-absolute-dates']; selector = '.relative-and-absolute-dates'; title='Use Both Absolute and Relative Dates' app-screenshot-end]

---