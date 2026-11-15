Su Flex, le persone che possono amministrare o moderare il tuo account vengono fatturate come posti mensili (seat). Questa è la parte di una fattura Flex che più spesso sorprende gli utenti, di solito perché i posti sono stati creati senza che nessuno se ne accorgesse.

### Cosa Conta Come Un Posto

- **Admin**: utenti nella pagina `Users` del tuo account con qualsiasi permesso amministrativo (fatturazione, personalizzazione, analisi, moderazione e così via). Il primo admin, normalmente il proprietario dell'account, è gratuito; ogni admin aggiuntivo è un posto.
- **Moderatori**: persone nella pagina `Moderators` che hanno accettato il loro invito. Un invito in sospeso non viene fatturato. Un utente il cui unico permesso è la moderazione dei commenti viene fatturato come moderatore, non come admin.
- **SSO Admin** e **SSO Moderatori**: utenti SSO con cui il tuo sito effettua l'accesso con `isAccountOwner`, `isAdminAdmin` o `isCommentModeratorAdmin` impostati nel payload SSO. Vengono fatturati alle stesse tariffe degli admin e moderatori normali. Vedi [Struttura Utente SSO](/guide-api.html#sso-user-structure).

I posti sono deduplicati per indirizzo email, quindi una persona che è sia admin che moderatore, o che ha sia un utente SSO che un accesso alla dashboard con la stessa email, viene fatturata una sola volta, alla tariffa più alta. Qualcuno con due indirizzi email diversi conta come due posti.

Sugli account fatturati in base agli utenti attivi mensili, un moderatore SSO che non ha effettuato l'accesso durante il mese non viene fatturato per quel mese.

### Posti Che Non Hai Aggiunto

Due funzionalità creano posti automaticamente:

- **Sincronizzazione admin SSO.** Quando un utente SSO effettua l'accesso con un'email che corrisponde a uno dei tuoi utenti della dashboard, i permessi di admin o moderatore di quell'utente vengono copiati sull'utente SSO. Con WordPress SSO questo significa che ogni amministratore di WordPress può diventare un admin o moderatore FastComments. Disattivala in `Customize` con l'opzione `Disable Automatic Admin Migration` sulla tua regola di personalizzazione.
- **Email diverse per la stessa persona.** Se il tuo accesso alla dashboard usa un'email e il tuo utente SSO un'altra, sono due posti. Usare la stessa email su entrambi li unisce.

### Verifica E Rimozione Dei Posti

La pagina [Billing Analytics](https://fastcomments.com/auth/my-account/analytics/billing) elenca i conteggi di `Moderators`, `SSO Admins` e `SSO Moderators`, e l'anteprima della prossima fattura mostra quanto costano. Rimuovi le persone di cui non hai bisogno dalle pagine `Users` e `Moderators`, oppure rimuovi i flag di admin dal tuo payload SSO. I posti vengono conteggiati quando la fattura viene creata alla fine del mese, quindi un posto rimosso prima di allora non viene fatturato per quel mese.

Vedi [Aggiungere Moderatori](/guide-users.html#adding-moderators) e [Aggiornare I Permessi Di Amministratore](/guide-users.html#updating-administrator-permissions) per come gestire entrambi.