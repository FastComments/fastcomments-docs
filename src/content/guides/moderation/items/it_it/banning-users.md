Ci sono due modi per impedire agli utenti di commentare sul tuo sito con FastComments.

Il primo è se conosci già la loro email, puoi inserirla nella pagina <a href="/auth/my-account/moderate-comments/banned-users" target="_blank">utenti bannati</a>.

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users'; selector = '.content .account-block'; title='The Banned Users Page' app-screenshot-end]

Questa pagina è accessibile tramite Modera commenti -> Utenti bannati

Quando procediamo a bannare un utente, possiamo scegliere il tipo: Permanente o Shadow ban permanente:

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users/new'; selector = '.content .account-block'; title='Banning a User' app-screenshot-end]

Il secondo modo per bannare un utente è cliccare sul pulsante di ban presente in ogni commento nella pagina di moderazione dei commenti.

Quando clicchiamo il pulsante di ban, verranno presentate alcune opzioni, in cui possiamo specificare il tipo e la durata del ban.

### Shadow Bans

Lo shadow-ban è un tipo di ban che fa sembrare che il commento o il voto dell'utente sia stato salvato con successo, quando in realtà non lo è stato. Questo può essere desiderabile in alcune situazioni.

### Bannare tramite indirizzo IP

A meno che un tenant non scelga di rinunciare, FastComments supporta il ban tramite IP memorizzando una versione hash dell'indirizzo IP del commentatore.

---