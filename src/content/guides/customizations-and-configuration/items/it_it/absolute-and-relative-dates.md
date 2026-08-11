[related-parameter-start name = 'absoluteAndRelativeDates'; type = 'boolean'; related-parameter-end]

Per impostazione predefinita, vengono utilizzate date relative localizzate. Ad esempio, accanto a un commento appena pubblicato potresti vedere "11 minuti fa".

Potrebbe essere necessario o desiderato mantenere questo formato di data relativa, ma anche mostrare la data completa accanto ad essa; in tal caso imposti questo parametro su true. 

[code-example-start config = {absoluteAndRelativeDates: true}; linesToHighlight = [6]; title = 'Use Both Absolute and Relative Dates'; code-example-end]

Questo può essere personalizzato senza codice, nella pagina di personalizzazione del widget, sotto Opzioni avanzate. Dovrai prima abilitare le Date assolute per vedere questa opzione nell'interfaccia utente.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.absolute-dates', '.relative-and-absolute-dates']; selector = '.relative-and-absolute-dates'; alt='Opzioni avanzate nella pagina di personalizzazione del widget con entrambe le date assolute e l\'impostazione combinata di data relativa abilitata'; title='Usa sia le date assolute che quelle relative' app-screenshot-end]