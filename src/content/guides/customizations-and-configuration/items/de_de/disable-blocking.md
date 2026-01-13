[related-parameter-start name = 'disableBlocking'; type = 'boolean'; related-parameter-end]

Standardmäßig erlaubt FastComments Nutzern, andere Nutzer zu blockieren. Wenn ein Nutzer blockiert wird, werden seine Kommentare
ausgeblendet, Benachrichtigungen zwischen den Nutzern verhindert und so weiter.

Es kann wünschenswert sein, diese Funktionalität zu deaktivieren. Das kann wie folgt gemacht werden:

[code-example-start config = {disableBlocking: true}; linesToHighlight = [6]; title = 'Disable Blocking'; code-example-end]

Dies kann auch ohne Code über die UI zur Widget-Anpassung vorgenommen werden, wodurch zudem eine ordnungsgemäße serverseitige Validierung aktiviert wird:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-blocking']; selector = '.disable-blocking'; title='Disable Blocking' app-screenshot-end]

---