[related-parameter-start name = 'absoluteAndRelativeDates'; type = 'boolean'; related-parameter-end]

Per impostazione predefinita vengono utilizzate date relative localizzate. Ad esempio, accanto a un commento inserito di recente potresti vedere "11 minuti fa".

Potrebbe essere necessario o desiderabile mantenere questo formato di data relativa, ma mostrare anche la data completa accanto ad essa; in tal caso imposta questo parametro su true. 

[code-example-start config = {absoluteAndRelativeDates: true}; linesToHighlight = [6]; title = 'Use Both Absolute and Relative Dates'; code-example-end]

Questo può essere personalizzato senza codice, nella pagina di personalizzazione del widget, sotto Opzioni avanzate. Dovrai prima abilitare Date assolute per vedere questa opzione nella UI.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.absolute-dates', '.relative-and-absolute-dates']; selector = '.relative-and-absolute-dates'; title='Use Both Absolute and Relative Dates' app-screenshot-end]

---