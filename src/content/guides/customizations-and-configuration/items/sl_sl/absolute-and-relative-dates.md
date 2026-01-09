[related-parameter-start name = 'absoluteAndRelativeDates'; type = 'boolean'; related-parameter-end]

Privzeto se uporabljajo lokalizirani relativni datumi. Na primer, zraven nedavno oddanega komentarja boste morda videli "pred 11 minutami".

Morda je potrebno ali zaželeno ohraniti ta relativni format datuma, hkrati pa prikazati tudi polni datum; v tem primeru nastavite ta parameter na true. 

[code-example-start config = {absoluteAndRelativeDates: true}; linesToHighlight = [6]; title = 'Uporabi absolutne in relativne datume'; code-example-end]

To lahko prilagodite brez kode na strani za prilagajanje gradnika, pod Napredne možnosti. Najprej boste morali omogočiti Absolutne datume, da boste to možnost videli v uporabniškem vmesniku.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.absolute-dates', '.relative-and-absolute-dates']; selector = '.relative-and-absolute-dates'; title='Uporabi absolutne in relativne datume' app-screenshot-end]