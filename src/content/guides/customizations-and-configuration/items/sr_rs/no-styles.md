[related-parameter-start name = 'noStyles'; type = 'boolean'; related-parameter-end]

Za veće projekte prilagođenog stilizovanja, može biti poželjno početi od nule i uopšte ne koristiti podrazumevane stilove.

Svo podrazumevano stilizovanje može se ukloniti postavljanjem parametra **noStyles** na true, na sledeći način:

[code-example-start config = {noStyles: true}; linesToHighlight = [6]; title = 'Onemogućavanje svih podrazumevanih stilova'; code-example-end]

Ovo se može prilagoditi bez koda, na stranici za prilagođavanje widgeta, pod naprednim opcijama:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.disable-all-default-styling']; selector = '.disable-all-default-styling'; alt='Polje za onemogućavanje svih podrazumevanih stilova omogućeno pod naprednim opcijama na stranici za prilagođavanje widgeta'; title='Onemogućavanje svih podrazumevanih stilova' app-screenshot-end]