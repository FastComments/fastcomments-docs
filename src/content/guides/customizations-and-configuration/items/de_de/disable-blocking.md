[related-parameter-start name = 'disableBlocking'; type = 'boolean'; related-parameter-end]

Standardmäßig erlaubt FastComments den Benutzern, andere Benutzer zu blockieren. Das Blockieren eines Benutzers führt dazu, dass deren Kommentare maskiert werden, verhindert Benachrichtigungen zwischen den Benutzern und so weiter.

Es kann wünschenswert sein, diese Funktion zu deaktivieren. So kann es durchgeführt werden:

[code-example-start config = {disableBlocking: true}; linesToHighlight = [6]; title = 'Disable Blocking'; code-example-end]

Dies kann auch ohne Code durchgeführt werden, was zudem eine ordnungsgemäße serverseitige Validierung ermöglicht, über die Widget-Anpassungsoberfläche:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-blocking']; selector = '.disable-blocking'; alt='Option zum Deaktivieren des Blockierens in der Widget-Anpassungsoberfläche, die verhindert, dass Benutzer einander blockieren'; title='Blockieren deaktivieren' app-screenshot-end]