Ci sono due modi per vietare agli utenti di commentare sul tuo sito con FastComments.

Il primo è se conosci già il loro indirizzo email: puoi inserirlo nella pagina degli <a href="https://fastcomments.com/auth/my-account/moderate-comments/banned-users" target="_blank">utenti bannati</a>.

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users'; selector = '.content .account-block'; alt='Elenco degli utenti bannati nella sezione Moderate Comments, con gli indirizzi email bannati e un pulsante per aggiungere un nuovo ban'; title='La pagina degli utenti bannati' app-screenshot-end]

Questa pagina è accessibile tramite Moderate Comments -> Banned Users

Quando vietiamo a un utente, possiamo scegliere un tipo, ovvero Permanent o Permanent Shadow Ban:

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users/new'; selector = '.content .account-block'; alt='Modulo di nuovo ban con un campo email e una scelta del tipo di ban tra Permanent o Permanent Shadow Ban'; title='Vietare a un utente' app-screenshot-end]

Il secondo modo per vietare a un utente è cliccando sul pulsante di ban che si trova su ogni commento nella pagina di Moderazione dei Commenti.

Quando clicchiamo sul pulsante di ban, ti verranno presentate alcune opzioni, dove possiamo specificare il tipo e la durata del ban.

### Alias email

Quando si vieta un utente tramite email, FastComments ignora automaticamente gli alias con `+`. Ad esempio, vietare `user+alias@gmail.com` vieterà
anche `user@gmail.com` e qualsiasi altra variazione con `+` di quell'indirizzo, come `user+other@gmail.com`.

### Shadow Ban

Uno shadow-ban è un tipo di ban che fa apparire come se il commento o il voto dell'utente fosse stato salvato con successo, quando in realtà non lo è stato. Questo può essere
desiderabile in certe situazioni.

### Ban tramite indirizzo IP

A meno che un tenant non voglia disattivarlo, FastComments supporta il ban tramite IP memorizzando una versione cifrata dell'indirizzo IP del commentatore.

### Ricerca degli utenti bannati

Una volta che la tua lista cresce oltre una o due pagine, puoi restringerla con la riga di ricerca sopra la tabella.

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users'; selector = '.content .filter-form'; alt='Riga di ricerca nella pagina degli utenti bannati con un menu a tendina Search By, un menu a tendina Match e un campo Value'; title='Ricerca degli utenti bannati' app-screenshot-end]

Ci sono tre controlli:

- **Search By** sceglie in quale campo cercare: Any Field, Email, Name, Banned By o Banned For Saying. Gli ultimi quattro corrispondono alle colonne con lo stesso nome nella tabella.
- **Match** sceglie come confrontare. **Contains** trova il tuo valore in qualsiasi punto del campo, e **Equals** corrisponde all'intero campo.
- **Value** è il testo da cercare.

Ogni campo viene confrontato senza distinzione tra maiuscole e minuscole, quindi cercando `SPAMMER@EXAMPLE.COM` si trova un ban memorizzato come `spammer@example.com`.

Alcune cose utili da sapere:

- **Banned For Saying** cerca il testo del commento che ha causato il ban dell'utente. Questo è il modo per trovare tutti quelli bannati per una determinata frase.
- **Banned By** cerca il nome del moderatore che ha emesso il ban, il che è utile per rivedere le decisioni di un altro moderatore.
- I ban con wildcard sono memorizzati con il loro `*`, quindi una ricerca **Contains** per `bademail.com` trova un ban `*@bademail.com`.
- **Name** corrisponde al nome mostrato nella colonna Name, quindi trova un utente anche se ha cambiato nome dopo essere stato bannato, e anche se hai creato il ban inserendo un indirizzo email senza che fosse registrato un nome al momento. Anche il nome registrato sul ban corrisponde comunque, quindi cercando sia il vecchio che il nome attuale si ottiene un risultato.
- **Any Field** cerca insieme email, nome, moderatore che ha emesso il ban e testo del commento bannato.

La tua ricerca fa parte dell'URL della pagina, quindi puoi condividere una lista filtrata con altri moderatori nello stesso modo in cui condividi altri link di moderazione. Sfogliando i risultati la ricerca rimane applicata, avviare una nuova ricerca ti riporta alla prima pagina, e **Clear** riporta alla lista completa.