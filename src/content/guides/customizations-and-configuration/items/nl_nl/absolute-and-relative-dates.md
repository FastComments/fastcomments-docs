[related-parameter-start name = 'absoluteAndRelativeDates'; type = 'boolean'; related-parameter-end]

Standaard worden gelokaliseerde relatieve datums gebruikt. Bijvoorbeeld, naast een recent geplaatst commentaar kun je "11 minuten geleden" zien.

Het kan nodig of gewenst zijn om dit relatieve datumformaat te behouden, maar ook de volledige datum ernaast te tonen; in dat geval stel je deze parameter in op true. 

[code-example-start config = {absoluteAndRelativeDates: true}; linesToHighlight = [6]; title = 'Use Both Absolute and Relative Dates'; code-example-end]

Dit kan zonder code worden aangepast op de widget-aanpassingspagina, onder Geavanceerde opties. Je moet eerst Absoluut datums inschakelen om deze optie in de UI te zien.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.absolute-dates', '.relative-and-absolute-dates']; selector = '.relative-and-absolute-dates'; alt='Geavanceerde opties op de widget-aanpassingspagina met zowel absolute datums als de gecombineerde relatieve datuminstelling ingeschakeld'; title='Gebruik zowel absolute als relatieve datums' app-screenshot-end]