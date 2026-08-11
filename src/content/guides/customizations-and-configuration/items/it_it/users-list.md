[related-parameter-start name = 'usersListLocation'; type = 'number'; related-parameter-end]
[related-parameter-start name = 'usersListIncludeOffline'; type = 'boolean'; related-parameter-end]

Per impostazione predefinita, FastComments non mostra un elenco di utenti nella pagina.

È possibile visualizzare un elenco di persone che stanno visualizzando la pagina in tempo reale, accanto al widget dei commenti. L'elenco si aggiorna in tempo reale man mano che gli utenti entrano e escono, e mostra il loro nome, avatar e un indicatore online.

Ci sono tre opzioni di layout:

- `1` - Top: una riga orizzontale di avatar sovrapposti visualizzata sopra i commenti.
- `2` - Left: una barra laterale con nomi e punti online visualizzata a sinistra del widget.
- `3` - Right: la stessa barra laterale visualizzata a destra del widget.

Imposta il flag **usersListLocation** per abilitare la funzionalità:

[code-example-start config = {usersListLocation: 3}; linesToHighlight = [6]; title = 'Mostra l\'elenco utenti a destra'; code-example-end]

Per impostazione predefinita l'elenco mostra solo gli utenti attualmente online. Per includere anche le persone che hanno commentato la pagina in passato (ma non la stanno visualizzando al momento), imposta **usersListIncludeOffline** su true:

[code-example-start config = {usersListLocation: 3, usersListIncludeOffline: true}; linesToHighlight = [6, 7]; title = 'Includi commentatori passati'; code-example-end]

I commentatori passati vengono visualizzati senza il punto verde online, così è chiaro chi è presente in questo momento.

Gli utenti con profili privati sono mostrati con un avatar generico e un'etichetta "Profilo Privato", in modo che il conteggio rimanga accurato senza rivelare le identità.

Questa impostazione può essere configurata anche senza codice. Nella pagina di personalizzazione del widget, vedi l'opzione "Users List Location". Quando la posizione è impostata su qualsiasi valore diverso da Off, appare una casella di controllo "Include past commenters" sotto di essa.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-settings'; alt='Posizione dell\'elenco utenti impostata a destra, con la casella di controllo includi commentatori passati mostrata sotto'; title='Impostazioni elenco utenti'; actions=[{type: 'set-value', selector: '#users-list-location-input', value: '3'}] app-screenshot-end]

Con 500 utenti live, l'elenco può essere fino a 30 secondi fuori data.