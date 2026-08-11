[related-parameter-start name = 'locale'; type = 'string'; related-parameter-end]

Per impostazione predefinita, FastComments renderizza il widget dei commenti nella lingua determinata dal sistema e dal browser dell'utente.

Quando un utente commenta o effettua il login, aggiorniamo la sua ultima lingua utilizzata e la usiamo anche per l'invio delle email.

Ciò influisce su come il widget dei commenti viene tradotto per l'utente. La lingua comprende la lingua e la regione dell'utente, quindi la configurazione della lingua solitamente cambia la lingua usata per mostrare il testo all'utente.

#### Via The UI

Questo può essere definito utilizzando l'interfaccia di personalizzazione del widget. Vedi l'opzione "Locale / Language":

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.locale-override'; alt='Menu a discesa Locale / Language nella pagina di personalizzazione del widget usato per sovrascrivere la lingua rilevata del visitatore'; title='Changing The Locale / Language' app-screenshot-end]

#### Via Code

Questo può essere sovrascritto con una lingua desiderata.

[code-example-start config = {locale: 'ru_ru'}; linesToHighlight = [6]; title = 'Manually Defining the User\'s Locale'; code-example-end]

### Supported Languages and Locale Codes

[Puoi trovare l'elenco completo delle lingue supportate e i relativi codici locale qui.](/guide-supported-languages.html#supported-languages)

### SSO Note

Se stai usando SSO, potresti voler passare la lingua dell'utente nell'oggetto user, in modo che le email e altre cose siano localizzate correttamente per loro.