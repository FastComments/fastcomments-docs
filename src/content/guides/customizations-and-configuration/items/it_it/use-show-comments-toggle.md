---
[related-parameter-start name = 'useShowCommentsToggle'; type = 'boolean'; related-parameter-end]

Per impostazione predefinita, FastComments renderizza la casella di inserimento commenti e il thread dei commenti contemporaneamente. Per risparmiare spazio verticale,
nasconderà anche eventuali altri campi obbligatori fino a quando l'utente non interagisce con il widget.

Tuttavia, il widget dei commenti può essere nascosto dietro un pulsante, per esempio:

[app-screenshot-start width=700; url=`https://fastcomments.com/embed?config=%7B%22tenantId%22%3A%22L177BUDVvSe%22%2C%22useShowCommentsToggle%22%3A%22true%22%2C%22urlId%22%3A%22file%253A%252F%252F%252FC%253A%252FUsers%252Fwinrid%252FAppData%252FRoaming%252FJetBrains%252FWebStorm2021.2%252Fscratches%252Fscratch_295.html%22%2C%22url%22%3A%22file%253A%252F%252F%252FC%253A%252FUsers%252Fwinrid%252FAppData%252FRoaming%252FJetBrains%252FWebStorm2021.2%252Fscratches%252Fscratch_295.html%22%2C%22pageTitle%22%3A%22%22%2C%22instanceId%22%3A%220.1281898364813452.1655790389169%22%7D&wId=comment-ui-v2`; selector = '.fast-comments'; delay=2000; title='Click to Show Comments' app-screenshot-end]

Il pulsante usa diversi testi tradotti a seconda che i commenti siano attualmente visualizzati o meno. Se i commenti sono nascosti, utilizza `translations.SHOW_COMMENTS_BUTTON_TEXT`. Se i commenti sono visualizzati, utilizza `translations.HIDE_COMMENTS_BUTTON_TEXT`. Le traduzioni possono contenere il testo `[count]` che verrà sostituito con il conteggio localizzato.

[code-example-start config = {useShowCommentsToggle: true}; linesToHighlight = [6]; title = 'Click to Show or Hide Comments'; code-example-end]

Questo è progettato per sostituire la configurazione `hideCommentsUnderCountTextFormat`.

Il conteggio viene aggiornato in tempo reale insieme al thread dei commenti. Il pulsante non viene mostrato se non ci sono commenti.

Questo può essere abilitato senza codice creando una regola di personalizzazione e attivando "Click to Show Comments":

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.click-to-show-comments']; selector = '.click-to-show-comments'; title='Enable Click to Show Comments' app-screenshot-end]


---