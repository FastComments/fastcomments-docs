[related-parameter-start name = 'usersListLocation'; type = 'number'; related-parameter-end]
[related-parameter-start name = 'usersListIncludeOffline'; type = 'boolean'; related-parameter-end]

Per impostazione predefinita, FastComments non mostra un elenco di utenti nella pagina.

Puoi visualizzare un elenco delle persone che stanno attualmente visualizzando la pagina, accanto al widget dei commenti. L'elenco si aggiorna in tempo reale quando gli utenti si connettono e si disconnettono, e mostra il loro nome, avatar e un indicatore di presenza online.

Sono disponibili tre opzioni di layout:

- `1` - In alto: una riga orizzontale di avatar sovrapposti visualizzata sopra i commenti.
- `2` - A sinistra: una barra laterale con nomi e indicatori online visualizzata a sinistra del widget.
- `3` - A destra: la stessa barra laterale visualizzata a destra del widget.

Imposta il flag **usersListLocation** per abilitare la funzionalità:

[code-example-start config = {usersListLocation: 3}; linesToHighlight = [6]; title = 'Show Users List on the Right'; code-example-end]

Per impostazione predefinita l'elenco mostra solo gli utenti attualmente online. Per includere anche le persone che hanno commentato la pagina in passato (ma non la stanno visualizzando attualmente), imposta **usersListIncludeOffline** su true:

[code-example-start config = {usersListLocation: 3, usersListIncludeOffline: true}; linesToHighlight = [6, 7]; title = 'Include Past Commenters'; code-example-end]

I commentatori passati vengono mostrati senza il puntino verde di presenza, in modo che sia chiaro chi è presente in questo momento.

Gli utenti con profili privati vengono mostrati con un avatar generico e un'etichetta "Profilo privato" in modo che il conteggio rimanga accurato senza rivelare le identità.

Questo può anche essere configurato senza codice. Nella pagina di personalizzazione del widget, vedi l'opzione "Posizione elenco utenti". Quando la posizione è impostata su qualsiasi valore diverso da "Disattivato", sotto di essa compare una casella di controllo "Includi commentatori passati".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-settings'; title='Users List Settings'; actions=[{type: 'set-value', selector: '#users-list-location-input', value: '3'}] app-screenshot-end]

---