[related-parameter-start name = 'absoluteDates'; type = 'boolean'; related-parameter-end]

Standaard worden gelokaliseerde relatieve datums gebruikt. Bijvoorbeeld, naast een recent geplaatste reactie zie je mogelijk "11 minuten geleden".

Het kan nodig of wenselijk zijn absolute datums te gebruiken; in dat geval zet je deze parameter op true. 

[code-example-start config = {absoluteDates: true}; linesToHighlight = [6]; title = 'Use Absolute Dates'; code-example-end]

Dit kan zonder code aangepast worden op de pagina voor het aanpassen van de widget, onder Geavanceerde opties:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.absolute-dates']; selector = '.absolute-dates'; title='Use Absolute Dates' app-screenshot-end]