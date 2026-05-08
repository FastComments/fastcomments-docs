[related-parameter-start name = 'usersListLocation'; type = 'number'; related-parameter-end]
[related-parameter-start name = 'usersListIncludeOffline'; type = 'boolean'; related-parameter-end]

Per impostazione predefinita, FastComments non mostra un elenco di utenti nella pagina.

È possibile visualizzare un elenco delle persone che stanno visualizzando la pagina, accanto al widget dei commenti. L'elenco si aggiorna in tempo reale quando gli utenti si uniscono o lasciano la pagina, e mostra il loro nome, avatar e un indicatore di presenza online.

Ci sono tre opzioni di layout:

- `1` - In alto: una fila orizzontale di avatar sovrapposti visualizzata sopra i commenti.
- `2` - A sinistra: una barra laterale con nomi e puntini online visualizzata a sinistra del widget.
- `3` - A destra: la stessa barra laterale visualizzata a destra del widget.

Imposta il flag **usersListLocation** per abilitare la funzionalità:

[code-example-start config = {usersListLocation: 3}; linesToHighlight = [6]; title = 'Show Users List on the Right'; code-example-end]

Per impostazione predefinita l'elenco mostra solo gli utenti attualmente online. Per includere anche le persone che hanno commentato la pagina in passato (ma non la stanno visualizzando al momento), impostare **usersListIncludeOffline** su true:

[code-example-start config = {usersListLocation: 3, usersListIncludeOffline: true}; linesToHighlight = [6, 7]; title = 'Include Past Commenters'; code-example-end]

I commentatori passati vengono visualizzati senza il puntino verde di stato online, in modo che sia chiaro chi è presente in questo momento.

Gli utenti con profili privati sono mostrati con un avatar generico e un'etichetta "Profilo privato" in modo che il conteggio rimanga accurato senza rivelare le identità.

Questo può anche essere configurato senza codice. Nella pagina di personalizzazione del widget, vedi l'opzione "Posizione elenco utenti". Quando la posizione è impostata su qualsiasi valore diverso da Off, sotto compare una casella di controllo "Includi commentatori passati".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-settings'; title='Users List Settings'; actions=[{type: 'set-value', selector: '#users-list-location-input', value: '3'}] app-screenshot-end]

Se ci sono più di 500 utenti live, l'elenco può risultare non aggiornato fino a 30 secondi.