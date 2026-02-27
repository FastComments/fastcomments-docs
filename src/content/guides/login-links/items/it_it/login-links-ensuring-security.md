Poiché i link di accesso sono essenzialmente password, prendiamo la sicurezza molto sul serio.

Tutti i link di accesso nel nostro sistema sono impostati per scadere dopo un certo periodo di tempo, e abbiamo anche meccanismi per rilevare
la possibilità di indovinare un link di accesso. Alcuni link di accesso sono suddivisi in più password, e se una viene indovinata,
l'altra sarà invalidata.

### Sicurezza rispetto alle password

Con la maggior parte dei sistemi che richiedono una password, puoi utilizzare il meccanismo 'Password dimenticata'
se disponi dell'email dell'utente. Questo significa, se hai accesso all'account email dell'utente,
non importa se il sistema sotto attacco usa password o link magici.

### Avvisi di accesso da nuovo IP

Quando un accesso avviene da un indirizzo IP che non è stato visto prima per un determinato account, FastComments invia un'email di avviso di sicurezza
con la posizione approssimativa e l'indirizzo IP. Questo aiuta gli utenti a rilevare accessi non autorizzati. Nota che FastComments non memorizza
gli indirizzi IP grezzi — viene conservata solo una forma offuscata per motivi di sicurezza.

### Email di backup per il recupero dell'account

Se perdi l'accesso alla tua email principale, puoi usare un'email di backup verificata per recuperare il tuo account. La tua email di backup funziona
con tutti i flussi di accesso. Puoi inserirla nella pagina per il recupero del nome utente, usarla con il login tramite link magici, o digitarla nel
campo username/email per il login con password.

Per configurare un'email di backup, vai su [Dettagli account](https://fastcomments.com/auth/my-account/edit-details) e clicca
**Definisci un'email di backup**. La tua email di backup è utilizzata solo per il recupero dell'account e non riceverà notifiche.

### Sicurezza rispetto a MFA

I link di accesso sono meno sicuri rispetto a MFA. FastComments ora supporta l'autenticazione a due fattori (2FA)
per gli account amministratore per offrire una sicurezza maggiore. Quando 2FA è abilitata, è richiesta anche quando si utilizzano i link di accesso.