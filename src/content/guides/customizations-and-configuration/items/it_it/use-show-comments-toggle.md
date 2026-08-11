[related-parameter-start name = 'useShowCommentsToggle'; type = 'boolean'; related-parameter-end]

Per impostazione predefinita, FastComments renderà la casella di inserimento del commento e il thread dei commenti contemporaneamente. Per risparmiare spazio verticale, nasconderà anche tutti gli altri campi richiesti finché il widget non verrà interagito.

Tuttavia, il widget dei commenti può essere nascosto dietro un pulsante, ad esempio:

[app-screenshot-start width=700; url=`https://fastcomments.com/embed?config=%7B%22tenantId%22%3A%22L177BUDVvSe%22%2C%22useShowCommentsToggle%22%3A%22true%22%2C%22urlId%22%3A%22file%253A%252F%252F%252FC%253A%252FUsers%252Fwinrid%252FAppData%252FRoaming%252FJetBrains%252FWebStorm2021.2%252Fscratches%252Fscratch_295.html%22%2C%22url%22%3A%22file%253A%252F%252F%252FC%253A%252FUsers%252Fwinrid%252FAppData%252FRoaming%252FJetBrains%252FWebStorm2021.2%252Fscratches%252Fscratch_295.html%22%2C%22pageTitle%22%3A%22%22%2C%22instanceId%22%3A%220.1281898364813452.1655790389169%22%7D&wId=comment-ui-v2`; selector = '.fast-comments'; delay=2000; alt='Widget dei commenti collassato dietro un pulsante che mostra il conteggio dei commenti finché un lettore non clicca'; title='Clicca per mostrare i commenti' app-screenshot-end]

Il pulsante utilizza testi tradotti diversi a seconda che i commenti siano attualmente mostrati o meno. Se i commenti sono nascosti, utilizza `translations.SHOW_COMMENTS_BUTTON_TEXT`. Se i commenti sono mostrati, utilizza `translations.HIDE_COMMENTS_BUTTON_TEXT`. Le traduzioni possono contenere il testo `[count]` che verrà sostituito dal conteggio localizzato.

[code-example-start config = {useShowCommentsToggle: true}; linesToHighlight = [6]; title = 'Clicca per mostrare o nascondere i commenti'; code-example-end]

Questo è progettato per sostituire la configurazione `hideCommentsUnderCountTextFormat`.

Il conteggio viene aggiornato in tempo reale con il thread dei commenti. Il pulsante non viene mostrato se non ci sono commenti.

Questo può essere abilitato senza codice creando una regola di personalizzazione e abilitando "Clicca per mostrare i commenti":

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.click-to-show-comments']; selector = '.click-to-show-comments'; alt='Casella di spunta per mostrare i commenti selezionata in una regola di personalizzazione nella pagina di personalizzazione del widget'; title='Abilita Clicca per mostrare i commenti' app-screenshot-end]