[related-parameter-start name = 'disableBlocking'; type = 'boolean'; related-parameter-end]

Per impostazione predefinita, FastComments consente agli utenti di bloccare altri utenti. Bloccare un utente farà sì che i suoi commenti siano mascherati, impedirà le notifiche tra gli utenti e così via.

Potrebbe essere utile disabilitare questa funzionalità. È possibile farlo in questo modo:

[code-example-start config = {disableBlocking: true}; linesToHighlight = [6]; title = 'Disable Blocking'; code-example-end]

Questo può essere fatto anche senza codice, il che abilita anche una corretta convalida lato server, tramite l'interfaccia di personalizzazione del widget:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-blocking']; selector = '.disable-blocking'; title='Disable Blocking' app-screenshot-end]