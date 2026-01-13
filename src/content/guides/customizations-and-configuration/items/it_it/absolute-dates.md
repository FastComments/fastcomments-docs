[related-parameter-start name = 'absoluteDates'; type = 'boolean'; related-parameter-end]

Per impostazione predefinita vengono utilizzate date relative localizzate. Ad esempio, accanto a un commento lasciato di recente potresti vedere "11 minuti fa".

Potrebbe essere necessario o desiderabile usare date assolute; in tal caso impostare questo parametro su true. 

[code-example-start config = {absoluteDates: true}; linesToHighlight = [6]; title = 'Use Absolute Dates'; code-example-end]

Questo può essere personalizzato senza codice, nella pagina di personalizzazione del widget, sotto Opzioni avanzate:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.absolute-dates']; selector = '.absolute-dates'; title='Use Absolute Dates' app-screenshot-end]

---