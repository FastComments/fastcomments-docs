[related-parameter-start name = 'absoluteAndRelativeDates'; type = 'boolean'; related-parameter-end]

Standaard worden gelokaliseerde relatieve datums gebruikt. Bijvoorbeeld, naast een recent geplaatste reactie zie je mogelijk "11 minuten geleden".

Het kan nodig of wenselijk zijn om dit relatieve datumformaat te behouden, maar daarnaast ook de volledige datum te tonen; in dat geval stelt u deze parameter in op true. 

[code-example-start config = {absoluteAndRelativeDates: true}; linesToHighlight = [6]; title = 'Use Both Absolute and Relative Dates'; code-example-end]

Dit kan zonder code worden aangepast op de pagina voor het aanpassen van de widget, onder Geavanceerde opties. U moet eerst Absolute Dates inschakelen om deze optie in de UI te zien.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.absolute-dates', '.relative-and-absolute-dates']; selector = '.relative-and-absolute-dates'; title='Use Both Absolute and Relative Dates' app-screenshot-end]

---