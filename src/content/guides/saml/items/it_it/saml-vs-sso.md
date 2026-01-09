FastComments offre sia l'autenticazione SSO che SAML. Comprendere le differenze ti aiuta a scegliere l'approccio giusto per la tua organizzazione.

### SSO Semplice/Sicuro Produzioni

FastComments offre due diversi flussi SSO per autenticarsi nel widget dei commenti tramite il tuo sito.
Questo è diverso da SAML e non richiede SAML. Invece, il Simple SSO richiede semplicemente di passare un oggetto al widget dei commenti, mentre il Secure SSO fa questo e in più esegue l'hashing del payload con una chiave API.

SAML, d'altra parte, autentica l'utente all'intero prodotto (in base ai loro permessi) *così come* al widget dei commenti
(se hanno i cookie di terze parti abilitati per il nostro dominio).

### Autenticazione SAML

SAML è un protocollo di autenticazione di livello enterprise che offre capacità di sicurezza e integrazione più robuste:

- **Implementation**: Richiede la configurazione dell'Identity Provider (IdP) e lo scambio dei certificati
- **Security**: Utilizza asserzioni XML firmate e supporta la crittografia
- **Use Case**: Ideale per aziende con infrastruttura SAML esistente (Active Directory, Okta, ecc.)
- **Setup Complexity**: Più complesso - richiede configurazione dell'IdP e gestione dei certificati
- **Enterprise Features**: Mappatura avanzata dei ruoli, gestione centralizzata degli utenti, audit trail

### Quando scegliere SAML

Considera l'autenticazione SAML se la tua organizzazione:

- Usa già un provider di identità compatibile con SAML (Okta, Azure AD, ADFS, ecc.)
- Richiede sicurezza e conformità di livello enterprise
- Ha bisogno di gestione centralizzata degli utenti e controllo degli accessi
- Ha più applicazioni che usano SAML per l'autenticazione
- Richiede audit trail dettagliati e reportistica di sicurezza

### Quando scegliere Simple o Secure SSO

Le nostre soluzioni SSO incentrate sul widget potrebbero essere sufficienti se tu:

- Hai un sistema di autenticazione personalizzato
- Hai bisogno di un'implementazione rapida con configurazione minima
- Non richiedi l'integrazione con provider di identità enterprise
- Vuoi controllare i dati degli utenti direttamente dalla tua applicazione
- Hai requisiti di sicurezza più semplici

Simple e Secure SSO sono comunemente usati per portali online, blog, ecc., dove l'utente ha già un account *tramite il tuo sito o la tua app*
ma non utilizza necessariamente SAML.