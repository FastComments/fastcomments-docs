[related-parameter-start name = 'disableBlocking'; type = 'boolean'; related-parameter-end]

Per impostazione predefinita, FastComments consente agli utenti di bloccare altri utenti. Il blocco di un utente farà sì che i suoi commenti vengano mascherati, impedirà le notifiche tra gli utenti e così via.

Potrebbe essere desiderabile disabilitare questa funzionalità. È possibile farlo in questo modo:

[code-example-start config = {disableBlocking: true}; linesToHighlight = [6]; title = 'Disabilita blocco'; code-example-end]

Questo può anche essere fatto senza codice, il che consente anche una corretta validazione lato server, tramite l'interfaccia di personalizzazione del widget:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-blocking']; selector = '.disable-blocking'; alt='Opzione per disabilitare il blocco nell\'interfaccia di personalizzazione del widget, che impedisce agli utenti di bloccarsi a vicenda'; title='Disabilita blocco' app-screenshot-end]