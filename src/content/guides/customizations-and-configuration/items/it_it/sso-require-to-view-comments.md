FastComments SSO (<a href="#sso">dettagli qui</a>) fornisce ai tuoi utenti un modo per commentare senza dover accedere a un'altra piattaforma.

Tuttavia, questo da solo non protegge i tuoi thread di commenti, poiché per impostazione predefinita i dati dei commenti sono informazioni pubblicamente disponibili: chiunque possa visualizzare la pagina può vedere i commenti.

Modificando un'impostazione, possiamo limitare il recupero dei commenti a meno che non lo faccia un amministratore o un utente SSO valido.

#### No-Code Setup

Possiamo impedire la visualizzazione e l'interazione con i nostri thread di commenti, quando SSO è configurato, creando una <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">regola di personalizzazione</a>.

Facendo ciò, cerca “SSO” e troverai questa opzione:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.require-sso'; selector = '.require-sso'; alt='Opzione Richiedi SSO per visualizzare i commenti abilitata in una regola di personalizzazione, con la scelta del livello di sicurezza'; title='Richiedi SSO per visualizzare i commenti' app-screenshot-end]

Abilitala e salva la regola di personalizzazione.

#### Only Protect a Certain Domain or Page

Per proteggere solo un determinato dominio o pagina, configureremo semplicemente la regola di personalizzazione di conseguenza.

Nella parte superiore dell'interfaccia di personalizzazione, troveremo due campi di input, Dominio e ID URL.

Per proteggere solo un dominio specifico, inserisci il dominio in questione nel campo “domain”.

Per proteggere una pagina specifica, inserisci l'URL della pagina nel campo “URL ID”. Se hai un'integrazione personalizzata con FastComments, puoi inserire qui un tipo di ID invece di un URL.

#### Security Levels

Quando richiedi SSO, dovrai decidere se richiedere Simple SSO o Secure SSO. Se scegli Simple SSO, entrambi sono consentiti, ma se scegli Secure SSO,
il contenuto deve essere recuperato con un payload Secure SSO hashato con la tua chiave API per poter essere visualizzato.

L'opzione del livello di sicurezza apparirà quando selezioni “Richiedi SSO per visualizzare i commenti”.

#### Protection Beyond Reading

Abilitare questa opzione proteggerà la pagina o il dominio dal ricevere commenti a meno che l'utente non sia autenticato tramite SSO.

#### Gotchas

Qualsiasi utente che ha creato commenti prima della tua integrazione SSO non potrà vederli, a meno che non effettui l'accesso tramite la tua integrazione SSO.