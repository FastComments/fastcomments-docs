Dopo aver configurato SAML in FastComments, è necessario configurare FastComments come Service Provider nel vostro identity provider.

### General IdP Configuration

La maggior parte degli identity provider richiede le seguenti informazioni per aggiungere FastComments come applicazione SAML:

#### Required Service Provider Information

Questi valori vengono generati automaticamente e mostrati nella pagina di configurazione SAML di FastComments:

**SP Entity ID / Audience**
- Format: `https://fastcomments.com/saml/{your-tenant-id}`
- Questo identifica in modo univoco la vostra istanza FastComments

**Assertion Consumer Service (ACS) URL**
- Format: `https://fastcomments.com/saml/callback/{your-tenant-id}`
- Dove il vostro IdP invia le risposte SAML dopo l'autenticazione

**SP Metadata URL** *(if supported by your IdP)*
- Format: `https://fastcomments.com/saml/metadata/{your-tenant-id}`
- Fornisce la configurazione SAML completa in formato XML

**SAML Login URL**
- Format: `https://fastcomments.com/saml/login/{your-tenant-id}`
- Link diretto per iniziare l'autenticazione SAML

### Required SAML Attributes

Configura il tuo identity provider per inviare questi attributi con le risposte SAML:

#### Essential Attributes

**Email Address** *(Required)*
- **Attribute Name**: `email`, `emailAddress`, or `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/emailaddress`
- **Purpose**: Identificazione univoca dell'utente e notifiche
- **Format**: Indirizzo email valido

#### Optional Attributes

**First Name**
- **Attribute Names**: `firstName`, `givenName`, or `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/givenname`
- **Purpose**: Nome visualizzato dell'utente

**Last Name**
- **Attribute Names**: `lastName`, `surname`, or `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/surname`
- **Purpose**: Cognome visualizzato dell'utente

**Roles** *(Important for access control)*
- **Attribute Names**: `roles`, `groups`, `memberOf`, or custom attribute names
- **Purpose**: Assegnazione ruoli e permessi in FastComments
- **Format**: Array di stringhe di ruolo o valori separati da virgola

### Common Identity Provider Configurations

#### Microsoft Azure AD

1. **Add Enterprise Application**
   - Cerca "FastComments" o crea una applicazione SAML personalizzata
   - Usa le informazioni SP fornite da FastComments

2. **Configure Attributes**
   - Email: `user.mail` or `user.userprincipalname`
   - First Name: `user.givenname`
   - Last Name: `user.surname`
   - Roles: `user.assignedroles` or directory groups

#### Okta

1. **Create SAML Application**
   - Usa "Create New App" e seleziona SAML 2.0
   - Configura con le informazioni SP di FastComments

2. **Attribute Statements**
   - Email: `user.email`
   - FirstName: `user.firstName`
   - LastName: `user.lastName`
   - Roles: `user.groups` or custom attributes

#### Google Workspace

1. **Add SAML Application**
   - Vai su Apps > Web and mobile apps > Add App > Add custom SAML app
   - Configura con le informazioni SP di FastComments

2. **Attribute Mapping**
   - Email: Primary email
   - First Name: First name
   - Last Name: Last name
   - Roles: Groups or custom attributes

#### Active Directory Federation Services (ADFS)

1. **Add Relying Party Trust**
   - Usa l'URL dei metadata di FastComments o la configurazione manuale
   - Configura le informazioni SP come fornite

2. **Claim Rules**
   - Email: Email Address claim
   - Name: Name ID claim
   - Roles: Group membership or custom claims

### Attribute Name Flexibility

FastComments accetta le informazioni sui ruoli da più nomi di attributo per adattarsi a diverse configurazioni di IdP:

- `roles`
- `groups`
- `memberOf`
- `role`
- `group`
- `http://schemas.microsoft.com/ws/2008/06/identity/claims/role`
- `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/role`

Questa flessibilità garantisce la compatibilità con vari identity provider senza richiedere convenzioni di denominazione specifiche per gli attributi.

### Testing Your Configuration

Dopo aver configurato il vostro identity provider:

1. Salva la configurazione IdP
2. Testa con un account utente dedicato per i test
3. Verifica che gli attributi vengano inviati correttamente
4. Controlla che i ruoli siano mappati correttamente
5. Assicurati che il flusso di autenticazione venga completato con successo

La maggior parte degli identity provider offre strumenti di test SAML per convalidare la configurazione prima di distribuirla agli utenti di produzione.