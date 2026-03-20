Ci sono due modi per impedire agli utenti di commentare sul tuo sito con FastComments.

Il primo è: se conosci già la loro email, puoi inserirla nella pagina <a href="https://fastcomments.com/auth/my-account/moderate-comments/banned-users" target="_blank">utenti bannati</a>.

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users'; selector = '.content .account-block'; title='The Banned Users Page' app-screenshot-end]

Questa pagina è accessibile tramite Moderazione commenti -> Utenti bannati

Quando si banna un utente, si può scegliere un tipo, oppure Permanente o Shadow ban permanente:

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users/new'; selector = '.content .account-block'; title='Banning a User' app-screenshot-end]

Il secondo modo per bannare un utente è cliccare il pulsante di ban che si trova su ogni commento nella pagina Moderazione commenti.

Quando si clicca il pulsante di ban, verranno presentate alcune opzioni, in cui è possibile specificare il tipo di ban e la durata.

### Shadow ban

Uno shadow-ban è un tipo di ban che fa sembrare che il commento o il voto dell'utente sia stato salvato correttamente, quando in realtà non lo è stato. Questo può essere desiderabile in determinate situazioni.

### Bannare tramite indirizzo IP

A meno che un tenant non scelga di rinunciare, FastComments supporta il ban tramite IP memorizzando una versione hash dell'indirizzo IP del commentatore.