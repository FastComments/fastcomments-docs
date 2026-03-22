Ci sono due modi per impedire agli utenti di commentare sul tuo sito con FastComments.

Il primo è, se conosci già la loro email, inserirla nella pagina <a href="https://fastcomments.com/auth/my-account/moderate-comments/banned-users" target="_blank">utenti bannati</a>.

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users'; selector = '.content .account-block'; title='The Banned Users Page' app-screenshot-end]

A questa pagina si può accedere tramite Moderate Comments -> Banned Users

Quando vogliamo bannare un utente, possiamo scegliere un tipo, permanente o Shadow Ban permanente:

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users/new'; selector = '.content .account-block'; title='Banning a User' app-screenshot-end]

Il secondo modo per bannare un utente è cliccare il pulsante di ban che si trova su ogni commento nella pagina Comment Moderation.

Quando clicchiamo il pulsante di ban, verranno mostrate alcune opzioni, dove possiamo specificare il tipo di ban e la durata.

### Email Aliases

Quando si banna un utente tramite email, FastComments ignora automaticamente gli alias con `+`. Ad esempio, bannare `user+alias@gmail.com` impedirà anche a `user@gmail.com` e a qualsiasi altra variazione con `+` di quell'indirizzo, come `user+other@gmail.com`.

### Shadow Bans

Uno shadow-ban è un tipo di ban che fa sembrare che il commento o il voto dell'utente sia stato salvato correttamente, quando in realtà non lo è. Questo può essere desiderabile in certe situazioni.

### Banning Via IP Address

A meno che un tenant non scelga di rinunciare, FastComments supporta il ban tramite IP memorizzando una versione hashata dell'indirizzo IP del commentatore.