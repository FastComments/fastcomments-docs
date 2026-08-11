Ci sono due modi per vietare agli utenti di commentare sul tuo sito con FastComments.

Il primo è se conosci già la loro email, puoi inserirla nella pagina <a href="https://fastcomments.com/auth/my-account/moderate-comments/banned-users" target="_blank">utenti bannati</a>.

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users'; selector = '.content .account-block'; alt='Elenco degli utenti bannati sotto Moderazione Commenti, con gli indirizzi email bannati e un pulsante per aggiungere un nuovo ban'; title='La pagina degli utenti bannati' app-screenshot-end]

Questa pagina è accessibile tramite Moderazione Commenti -> Utenti Bannati

Quando vogliamo bannare un utente, possiamo scegliere un tipo, sia Permanente che Ban Ombra Permanente:

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users/new'; selector = '.content .account-block'; alt='Nuovo modulo di ban con un campo email e una scelta del tipo di ban: Permanente o Ban Ombra Permanente'; title='Bannare un utente' app-screenshot-end]

Il secondo modo per bannare un utente è cliccando il pulsante di ban posizionato su ogni commento nella pagina di Moderazione Commenti.

Quando clicchiamo il pulsante di ban, ti verranno presentate alcune opzioni, dove possiamo specificare il tipo di ban e la durata.

### Alias Email

Quando si banna un utente tramite email, FastComments ignora automaticamente gli alias `+`. Per esempio, bannare `user+alias@gmail.com` bannerà anche `user@gmail.com` e qualsiasi altra variazione `+` di quell'indirizzo, come `user+other@gmail.com`.

### Ban Ombra

Un ban ombra è un tipo di ban che fa apparire che il commento o il voto dell'utente sia stato salvato con successo, quando in realtà non lo è stato. Questo può essere desiderabile in alcune situazioni.

### Bannare tramite indirizzo IP

A meno che un tenant non desideri rinunciare, FastComments supporta il ban tramite IP memorizzando una versione hash dell'indirizzo IP del commentatore.