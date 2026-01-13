[related-parameter-start name = 'locale'; type = 'string'; related-parameter-end]

Per impostazione predefinita, FastComments renderizzerà il widget dei commenti nella locale determinata dal sistema e dal browser dell'utente.

Quando un utente commenta o effettua il login, aggiorniamo la sua ultima locale utilizzata e la usiamo anche per l'invio delle email.

Questo influisce su come il widget dei commenti viene tradotto per l'utente. La locale consiste nella lingua e nella regione dell'utente, quindi configurare la locale di solito
cambierà la lingua usata per mostrare il testo all'utente.

#### Tramite l'interfaccia utente

Questo può essere definito usando l'interfaccia di personalizzazione del widget. Vedi l'opzione "Locale / Lingua":

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.locale-override'; title='Changing The Locale / Language' app-screenshot-end]

#### Tramite codice

Questo può essere sovrascritto con la locale desiderata.

[code-example-start config = {locale: 'ru_ru'}; linesToHighlight = [6]; title = 'Manually Defining the User\'s Locale'; code-example-end]

### Lingue supportate e codici locale

[Puoi trovare l'elenco completo delle lingue supportate e dei corrispondenti codici locale qui.](/guide-supported-languages.html#supported-languages)

### Nota SSO

Se stai usando SSO, potresti voler passare la locale dell'utente nell'oggetto user, affinché le email e altri elementi siano localizzati correttamente per loro.

---