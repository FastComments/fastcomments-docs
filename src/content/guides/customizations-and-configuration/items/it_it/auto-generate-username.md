Quando gli utenti commentano o votano senza essere autenticati, verrà loro chiesto di fornire la loro email e il loro nome utente.

Per alcuni siti, chiedere a un visitatore di inventare un nome utente univoco è un ostacolo, in particolare su mobile. FastComments può generare un nome utente neutro per ogni nuovo visitatore e precompilarlo nel campo del nome utente, ad esempio `BraveOtter4172`.

Il visitatore può lasciarlo così com'è, oppure sostituirlo con un nome a sua scelta.

Questa funzionalità può essere attivata dalla UI di Personalizzazione, nell'impostazione chiamata `Generate Usernames Automatically`:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.auto-generate-username'; alt='L\'opzione Generate Usernames Automatically nella UI di personalizzazione del widget'; title='Genera nomi utente automaticamente' app-screenshot-end]

#### Come si comporta

- Ogni nome generato è univoco. Viene verificato rispetto agli account esistenti e riservato per la sessione del browser di quel visitatore, quindi due visitatori non ricevono lo stesso nome.
- Il nome viene generato solo per i visitatori che non ne hanno ancora uno. Gli utenti autenticati, gli utenti SSO e i visitatori che hanno già commentato mantengono il loro nome esistente.
- Funziona con o senza i [commenti anonimi](/guide-customizations-and-configuration.html#allow-anon). Con i commenti anonimi disattivati, il visitatore inserisce comunque la sua email, ma non deve più inventare un nome utente.
- Un visitatore di ritorno che inserisce un'email già utilizzata in precedenza viene abbinato al suo account esistente e mantiene il nome su quell'account.
- Se è impostato anche un [Nome utente predefinito](/guide-customizations-and-configuration.html#default-username), il nome generato ha la precedenza.