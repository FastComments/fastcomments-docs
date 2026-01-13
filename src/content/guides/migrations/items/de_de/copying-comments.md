Falls Daten verschoben werden müssen, stellt FastComments ein Self-Service-Tool zum Verschieben von Kommentaren
zwischen Seiten und Artikeln zur Verfügung.

Here's what the comment copy page form looks like:

[app-screenshot-start url='/auth/my-account/manage-data/copy-comments'; selector = '.account-block'; title='The Copy Comment Form' app-screenshot-end]

### Filling out the "From" Fields

Um zu entscheiden, von wo Kommentare verschoben werden sollen, müssen wir einfach die Quell-`URL ID` kennen.

If you aren't passing a value for `urlId` in the comment widget configuration, then this will be a "clean" version of the page URL.

You can see what values your comments have for `URL ID` by exporting them.

### Filling out the "To" Fields

Um zu entscheiden, wohin Kommentare verschoben werden sollen, müssen wir die Ziel-`URL ID` und die `URL` kennen.

The `URL ID` will be the bucket that the comment goes in. The `URL` field is used so that you can navigate directly
zum Kommentar aus E-Mails und Moderationswerkzeugen.

#### WordPress

If you are using WordPress, you would for example enter the Article IDs in the To/From `URL ID` fields in the migration tool,
rather than a URL.