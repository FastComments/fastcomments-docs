[related-parameter-start name = 'usersListLocation'; type = 'number'; related-parameter-end]
[related-parameter-start name = 'usersListIncludeOffline'; type = 'boolean'; related-parameter-end]

Per impostazione predefinita, FastComments non mostra un elenco di utenti sulla pagina.

Puoi visualizzare un elenco di persone che stanno attualmente visualizzando la pagina, accanto al widget dei commenti. L'elenco si aggiorna in tempo reale man mano che gli utenti entrano e escono, e mostra il loro nome, avatar e un indicatore di presenza online.

Ci sono tre opzioni di layout:

- `1` - In alto: una fila orizzontale di avatar sovrapposti visualizzata sopra i commenti.
- `2` - Sinistra: una barra laterale con nomi e punti online visualizzata a sinistra del widget.
- `3` - Destra: la stessa barra laterale visualizzata a destra del widget.

Imposta la flag **usersListLocation** per abilitare la funzione:

[code-example-start config = {usersListLocation: 3}; linesToHighlight = [6]; title = 'Show Users List on the Right'; code-example-end]

Per impostazione predefinita, l'elenco mostra solo gli utenti attualmente online. Per includere anche le persone che hanno commentato in passato (ma che non stanno visualizzando la pagina in questo momento), imposta **usersListIncludeOffline** su true:

[code-example-start config = {usersListLocation: 3, usersListIncludeOffline: true}; linesToHighlight = [6, 7]; title = 'Include Past Commenters'; code-example-end]

I commentatori passati vengono mostrati senza il punto verde di online, in modo da capire chiaramente chi è presente in questo momento.

Gli utenti con profili privati vengono mostrati con un avatar generico e un'etichetta "Profilo privato" così il conteggio rimane accurato senza rivelare le identità.

Questo può anche essere configurato senza codice. Nella pagina di personalizzazione del widget, vedi l'opzione "Posizione elenco utenti":

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-location'; title='Users List Location' app-screenshot-end]

Quando la posizione è impostata su qualsiasi valore diverso da "Disattivato", la casella "Includi commentatori passati" viene mostrata sotto di essa:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-include-offline'; title='Include Past Commenters' app-screenshot-end]

---