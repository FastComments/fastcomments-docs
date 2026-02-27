Poiché i link di accesso sono sostanzialmente password, prendiamo la sicurezza molto seriamente.

Tutti i link di accesso nel nostro sistema sono impostati per scadere dopo un certo periodo di tempo, e disponiamo anche di meccanismi per rilevare i tentativi di indovinare un link di accesso. Alcuni link di accesso sono suddivisi in più password e, se una viene indovinata, le altre saranno invalidate.

### Sicurezza rispetto alle password

Con la maggior parte dei sistemi che richiedono una password, è possibile utilizzare una procedura "Password dimenticata" se si dispone dell'email dell'utente. Ciò significa che, se si ha accesso all'account email dell'utente, non importa se il sistema sotto attacco utilizza password o link magici.

### Avvisi di accesso da nuovo IP

Quando un accesso avviene da un indirizzo IP che non è stato visto prima per un determinato account, FastComments invia una e-mail di avviso di sicurezza con la posizione approssimativa e l'indirizzo IP. Questo aiuta gli utenti a rilevare accessi non autorizzati. Nota che FastComments non memorizza indirizzi IP grezzi — viene memorizzata solo una forma offuscata per motivi di sicurezza.

### Sicurezza rispetto all'autenticazione a più fattori (MFA)

I link di accesso sono meno sicuri dell'autenticazione a più fattori. FastComments ora supporta l'autenticazione a due fattori (2FA) per gli account admin per fornire una sicurezza migliorata. Quando la 2FA è abilitata, è richiesta anche quando si utilizzano link di accesso.