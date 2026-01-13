FastComments SSO (<a href="#sso">maggiori dettagli</a>) offre ai tuoi utenti un modo per commentare senza dover effettuare il login su un'altra piattaforma.

Tuttavia, questo da solo non mette al sicuro i thread dei commenti, poiché per impostazione predefinita i dati dei commenti sono informazioni pubbliche - chiunque possa visualizzare la pagina può vedere i commenti.

Modificando un'impostazione, possiamo limitare il recupero dei commenti a soli amministratori o utenti SSO validi.

#### No-Code Setup

Possiamo impedire la visualizzazione e l'interazione con i nostri thread dei commenti, quando SSO è configurato, creando una <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">regola di personalizzazione</a>.

Quando lo fai, cerca SSO e troverai questa opzione:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.require-sso'; selector = '.require-sso'; title='Require SSO To View Comments' app-screenshot-end]

Attiva e salva la regola di personalizzazione.

#### Only Protect a Certain Domain or Page

Per proteggere solo un determinato dominio o una pagina, configureremo semplicemente la regola di personalizzazione in tal senso.

Nella parte superiore dell'interfaccia di personalizzazione troveremo due campi, Domain and URL ID.

Per proteggere solamente un dominio specifico, inserisci il dominio in questione nel campo "domain".

Per proteggere una pagina particolare, inserisci l'URL della pagina nel campo "URL ID". Se hai un'integrazione personalizzata con FastComments, puoi inserire qui un tipo di ID invece di un URL.

#### Security Levels

Quando richiedi SSO, dovrai decidere se richiedere Simple SSO o Secure SSO. Se richiedi Simple SSO, allora entrambi sono consentiti, ma se richiedi Secure SSO allora il contenuto deve essere recuperato con un payload di Secure SSO la cui hash è calcolata con la tua API key affinché possa essere visualizzato.

L'opzione del livello di sicurezza apparirà quando selezioni "Require SSO To View Comments".

#### Protection Beyond Reading

Abilitando questa opzione, la pagina o il dominio saranno protetti dall'essere commentati a meno che l'utente non abbia effettuato l'accesso tramite SSO.

#### Gotchas

Qualsiasi utente che ha creato commenti prima della tua integrazione SSO non potrà vederli, a meno che non effettui l'accesso tramite la tua integrazione SSO.