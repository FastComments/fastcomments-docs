[related-parameter-start name = 'absoluteAndRelativeDates'; type = 'boolean'; related-parameter-end]

Podrazumevano se koriste lokalizovani relativni datumi. Na primer, pored nedavno ostavljenog komentara možete videti „pre 11 minuta“.

Može biti potrebno ili poželjno zadržati ovaj relativni format datuma, ali i prikazati puni datum pored njega, u kom slučaju postavljate ovaj parametar na true. 

[code-example-start config = {absoluteAndRelativeDates: true}; linesToHighlight = [6]; title = 'Koristite i apsolutne i relativne datume'; code-example-end]

Ovo se može prilagoditi bez koda, na stranici za prilagođavanje widgeta, pod naprednim opcijama. Prvo ćete morati da omogućite Apsolutne datume da biste videli ovu opciju u UI‑u.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.absolute-dates', '.relative-and-absolute-dates']; selector = '.relative-and-absolute-dates'; alt='Napredne opcije na stranici za prilagođavanje widgeta sa i apsolutnim datumima i kombinovanom postavkom relativnog datuma omogućenom'; title='Koristite i apsolutne i relativne datume' app-screenshot-end]