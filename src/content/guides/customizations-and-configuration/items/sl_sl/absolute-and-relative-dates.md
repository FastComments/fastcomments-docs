[related-parameter-start name = 'absoluteAndRelativeDates'; type = 'boolean'; related-parameter-end]

Privzeto se uporabljajo lokalizirani relativni datumi. Na primer, poleg nedavno objavljenega komentarja lahko vidite "11 minut nazaj".

Morda je potrebno ali zaželeno ohraniti ta relativni format datuma, hkrati pa prikazati tudi celoten datum poleg njega; v tem primeru nastavite ta parameter na true. 

[code-example-start config = {absoluteAndRelativeDates: true}; linesToHighlight = [6]; title = 'Uporabi tako absolutne kot relativne datume'; code-example-end]

To je mogoče prilagoditi brez kode na strani za prilagajanje gradnika, pod naprednimi možnostmi. Najprej boste morali omogočiti absolutne datume, da vidite to možnost v uporabniškem vmesniku.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.absolute-dates', '.relative-and-absolute-dates']; selector = '.relative-and-absolute-dates'; alt='Napredne možnosti na strani za prilagajanje gradnika z omogočenimi absolutnimi datumi in združenim nastavitvijo relativnega datuma'; title='Uporabi tako absolutne kot relativne datume' app-screenshot-end]