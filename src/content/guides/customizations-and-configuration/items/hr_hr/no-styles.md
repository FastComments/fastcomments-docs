[related-parameter-start name = 'noStyles'; type = 'boolean'; related-parameter-end]

Za veće projekte prilagođenog stiliziranja, možda je poželjno započeti s čistim početkom i uopće ne koristiti zadano stiliziranje.

Svo zadano stiliziranje može se ukloniti postavljanjem parametra **noStyles** na true, na sljedeći način:

[code-example-start config = {noStyles: true}; linesToHighlight = [6]; title = 'Disabling All Default Styles'; code-example-end]

Ovo se može prilagoditi bez koda, na stranici prilagodbe widgeta, pod Naprednim opcijama:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.disable-all-default-styling']; selector = '.disable-all-default-styling'; alt='Potvrdni okvir za onemogućavanje svih zadnjih stilova omogućena pod Naprednim opcijama na stranici prilagodbe widgeta'; title='Onemogućavanje svih zadnjih stilova' app-screenshot-end]