FastComments è una piattaforma localizzata. Tutti i nostri widget, email e notifiche sono localizzati.

Localizzato significa che mostriamo una lingua e una formattazione diverse, basate sulla posizione dell'utente
e sulla lingua preferita. Determiniamo questo in base alle informazioni che il browser dell'utente ci fornisce.

Possiamo personalizzare il testo nell'email andando nella scheda `Translations`, selezionando una `Locale`
e modificando il testo. Il testo modificato rispetto al valore predefinito è evidenziato nell'interfaccia utente. Puoi
passare tra le locale e salvare alla fine, senza perdere le modifiche.

Il testo localizzato è accessibile tramite l'oggetto `TEXT`, per esempio: `<%= TEXT.INTRO %>`.

### Nota SSO

Per le integrazioni SSO, se `locale` non è specificato, verrà aggiornato ogni volta che l'utente
accede al widget dei commenti con una locale diversa. Ciò significa che la loro preferenza linguistica
viene aggiornata automaticamente e le email future saranno inviate in quella locale.

Questo può anche essere impostato manualmente fornendo `locale` nel payload SSO.