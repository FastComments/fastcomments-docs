[related-parameter-start name = 'absoluteDates'; type = 'boolean'; related-parameter-end]

Per impostazione predefinita, vengono utilizzate date relative localizzate. Per esempio, accanto a un commento appena lasciato potresti vedere "11 minuti fa".

Potrebbe essere necessario o desiderato utilizzare date assolute, nel qual caso imposti questo parametro su true. 

[code-example-start config = {absoluteDates: true}; linesToHighlight = [6]; title = 'Use Absolute Dates'; code-example-end]

Questo può essere personalizzato senza codice, nella pagina di personalizzazione del widget, sotto Opzioni Avanzate:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.absolute-dates']; selector = '.absolute-dates'; alt='Opzioni avanzate nella pagina di personalizzazione del widget con l\'interruttore delle date assolute attivato'; title='Usa date assolute' app-screenshot-end]