[related-parameter-start name = 'absoluteAndRelativeDates'; type = 'boolean'; related-parameter-end]

Prema zadanim postavkama, koriste se lokalizirani relativni datumi. Na primjer, uz nedavno ostavljen komentar možete vidjeti "11 minuta prije".

Možda je potrebno ili poželjno zadržati ovaj format relativnog datuma, ali također prikazati puni datum uz njega, u kojem slučaju postavite ovaj parametar na true. 

[code-example-start config = {absoluteAndRelativeDates: true}; linesToHighlight = [6]; title = 'Koristite i apsolutne i relativne datume'; code-example-end]

Ovo se može prilagoditi bez koda, na stranici prilagodbe widgeta, pod Naprednim opcijama. Prvo ćete morati omogućiti Apsolutne datume da biste vidjeli ovu opciju u korisničkom sučelju.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.absolute-dates', '.relative-and-absolute-dates']; selector = '.relative-and-absolute-dates'; alt='Napredne opcije na stranici prilagodbe widgeta s oba apsolutna datuma i omogućenom kombiniranom postavkom relativnog datuma'; title='Koristite i apsolutne i relativne datume' app-screenshot-end]