[related-parameter-start name = 'showLiveRightAway'; type = 'boolean'; related-parameter-end]

Per impostazione predefinita, il commento live è abilitato. Ciò significa che se vengono aggiunti, eliminati, modificati o fissati dei commenti, le modifiche dovrebbero apparire
a tutti gli utenti che stanno visualizzando il thread dei commenti nello stesso momento.

Tuttavia, per impostazione predefinita questi nuovi commenti appariranno sotto un pulsante mostrato dinamicamente con un testo simile a "Mostra 2 nuovi commenti".

Se i nuovi commenti sono risposte dirette alla pagina, il pulsante verrà mostrato nella parte superiore del thread dei commenti. Se sono risposte a un commento particolare, 
il pulsante verrà mostrato sotto quel commento.

Questo serve per evitare che la dimensione della pagina cambi continuamente per l'utente, causando potenzialmente frustrazione quando si cerca di afferrare la barra di scorrimento.

Per alcuni casi d'uso, come le offerte in tempo reale o gli eventi online, questo comportamento non è desiderato - potresti voler che il widget dei commenti sia
più simile a una casella "chat" in cui i nuovi commenti "appaiono subito".

Da qui il nome della flag che abilita questa funzionalità: **showLiveRightAway**.

Possiamo attivarla come segue:

[code-example-start config = {showLiveRightAway: true}; linesToHighlight = [6]; title = 'Show Live Comments Right Away'; code-example-end]

Questa impostazione può essere personalizzata senza codice, nella pagina di personalizzazione del widget:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.collapse-live-comments'; selector = '.collapse-live-comments'; title='Show Live Comments Right Away' app-screenshot-end]

---