[related-parameter-start name = 'countAll'; type = 'boolean'; related-parameter-end]

Il conteggio dei commenti visualizzato nella parte superiore del widget dei commenti può mostrare o tutti i commenti "di primo livello", ovvero le risposte che sono risposte direttamente alla pagina o all'articolo stesso, oppure può essere un conteggio di **tutti** i commenti nidificati.

Per impostazione predefinita, questo è `true` - è un conteggio del secondo tipo - tutti i commenti. Nelle versioni precedenti del widget dei commenti il valore predefinito era `false`.

Possiamo cambiare il comportamento, così che sia un conteggio di **tutti** i commenti nidificati impostando il flag **countAll** su true.

[code-example-start config = {countAll: true}; linesToHighlight = [6]; title = 'Counting All Comments'; code-example-end]

Se volessimo che il conteggio riflettesse solo i commenti di primo livello, impostiamo il flag su false.

[code-example-start config = {countAll: false}; linesToHighlight = [6]; title = 'Counting Top Level Comments'; code-example-end]

Attualmente questo non può essere personalizzato senza modifiche al codice.

---