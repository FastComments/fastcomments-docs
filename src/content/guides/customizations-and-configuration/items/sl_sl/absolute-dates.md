[related-parameter-start name = 'absoluteDates'; type = 'boolean'; related-parameter-end]

Privzeto se uporabljajo lokalizirani relativni datumi. Na primer, poleg nedavno objavljenega komentarja lahko vidite "11 minut nazaj".

Morda bo potrebno ali zaželeno uporabiti absolutne datume, v tem primeru nastavite ta parameter na true. 

[code-example-start config = {absoluteDates: true}; linesToHighlight = [6]; title = 'Use Absolute Dates'; code-example-end]

To je mogoče prilagoditi brez kode, na strani za prilagajanje gradnika, pod naprednimi možnostmi:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.absolute-dates']; selector = '.absolute-dates'; alt='Napredne možnosti na strani za prilagajanje gradnika z vklopljenim preklopom absolutnih datumov'; title='Uporabi absolutne datume' app-screenshot-end]