[related-parameter-start name = 'showLiveRightAway'; type = 'boolean'; related-parameter-end]

Per impostazione predefinita, i commenti live sono abilitati. Ciò significa che se vengono aggiunti, eliminati, modificati o evidenziati commenti, le modifiche dovrebbero apparire a tutti gli utenti che visualizzano il thread dei commenti contemporaneamente.

Tuttavia, per impostazione predefinita, questi nuovi commenti appariranno sotto un pulsante mostrato dinamicamente con un testo simile a "Show 2 New Comments".

Se i nuovi commenti sono risposte direttamente alla pagina, il pulsante verrà mostrato in cima al thread dei commenti. Se sono risposte a un commento specifico, il pulsante verrà mostrato sotto quel commento.

Questo serve a evitare che la dimensione della pagina cambi costantemente per l'utente, potenzialmente causando frustrazione quando si tenta di afferrare la barra di scorrimento.

Per alcuni casi d'uso, come aste live o eventi online, questo non è il comportamento desiderato - potresti voler che il widget dei commenti sia più simile a una "chat" in cui i nuovi commenti "show right away".

Ecco il nome del flag che abilita questa funzionalità: **showLiveRightAway**.

Possiamo attivarlo come segue:

[code-example-start config = {showLiveRightAway: true}; linesToHighlight = [6]; title = 'Mostra i commenti live subito'; code-example-end]

Questo può essere personalizzato senza codice, nella pagina di personalizzazione del widget:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.collapse-live-comments'; selector = '.collapse-live-comments'; alt='Impostazione di collasso dei commenti live attivata così i nuovi commenti appaiono immediatamente invece che dietro un pulsante'; title='Mostra i commenti live subito' app-screenshot-end]