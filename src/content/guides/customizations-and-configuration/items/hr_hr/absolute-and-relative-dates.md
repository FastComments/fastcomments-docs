[related-parameter-start name = 'absoluteAndRelativeDates'; type = 'boolean'; related-parameter-end]

Po zadanoj postavci koriste se lokalizirani relativni datumi. Na primjer, uz nedavno objavljeni komentar možete vidjeti "prije 11 minuta".

Može biti potrebno ili poželjno zadržati ovaj relativni format datuma, ali također prikazati i puni datum pored njega — u tom slučaju postavite ovaj parametar na true. 

[code-example-start config = {absoluteAndRelativeDates: true}; linesToHighlight = [6]; title = 'Use Both Absolute and Relative Dates'; code-example-end]

Ovo se može prilagoditi bez koda, na stranici za prilagodbu widgeta, pod Advanced Options. Najprije ćete morati omogućiti Absolute Dates da biste vidjeli ovu opciju u korisničkom sučelju.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.absolute-dates', '.relative-and-absolute-dates']; selector = '.relative-and-absolute-dates'; title='Use Both Absolute and Relative Dates' app-screenshot-end]