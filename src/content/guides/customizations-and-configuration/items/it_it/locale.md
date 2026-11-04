[related-parameter-start name = 'locale'; type = 'string'; related-parameter-end]

Per impostazione predefinita, FastComments renderizzerà il widget dei commenti nella locale determinata dal sistema e dal browser dell'utente.

Quando un utente commenta o effettua il login, aggiorniamo la sua ultima locale utilizzata e la usiamo anche per l'invio delle email.

Questo influisce su come il widget dei commenti viene tradotto per l'utente. La locale è composta dalla lingua e dalla regione dell'utente, quindi configurare la locale di solito cambierà la lingua usata per mostrare il testo all'utente.

#### Tramite la UI

Questo può essere definito utilizzando la UI di personalizzazione del widget. Vedi l'opzione "Locale / Language":

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.locale-override'; alt='Menu a tendina Locale / Language nella pagina di personalizzazione del widget utilizzato per sovrascrivere la locale rilevata del visitatore'; title='Modifica della Locale / Lingua' app-screenshot-end]

#### Tramite Codice

Questo può essere sovrascritto con una locale desiderata.

[code-example-start config = {locale: 'ru_ru'}; linesToHighlight = [6]; title = 'Definizione Manuale della Locale dell\'Utente'; code-example-end]

### Lingue Supportate e Codici delle Locale

[Puoi trovare l'elenco completo delle lingue supportate e dei corrispondenti codici delle locale qui.](/guide-supported-languages.html#supported-languages)

### Nota su SSO

Se stai usando SSO, potresti voler passare la locale dell'utente nell'oggetto utente, in modo che le email e altre cose siano localizzate correttamente per lui.