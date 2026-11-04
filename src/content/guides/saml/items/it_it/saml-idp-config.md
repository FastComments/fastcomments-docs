Dopo aver configurato SAML in FastComments, è necessario configurare FastComments come Service Provider nel proprio identity provider.

### Configurazione generale dell'IdP

La maggior parte degli identity provider richiede le seguenti informazioni per aggiungere FastComments come applicazione SAML:

#### Informazioni richieste sul Service Provider

Questi valori vengono generati automaticamente e visualizzati nella pagina di configurazione SAML di FastComments:

**SP Entity ID / Audience**
- Formato: `https://fastcomments.com/saml/{your-tenant-id}`
- Identifica in modo univoco la propria istanza FastComments

**Assertion Consumer Service (ACS) URL**
- Formato: `https://fastcomments.com/saml/callback/{your-tenant-id}`
- Dove l'IdP invia le risposte SAML dopo l'autenticazione

**SP Metadata URL** *(se supportato dal proprio IdP)*
- Formato: `https://fastcomments.com/saml/metadata/{your-tenant-id}`
- Fornisce la configurazione SAML completa in formato XML

**SAML Login URL**
- Formato: `https://fastcomments.com/saml/login/{your-tenant-id}`
- Link diretto per avviare l'autenticazione SAML

### Attributi SAML richiesti

Configurare l'identity provider per inviare questi attributi con le risposte SAML:

#### Attributi essenziali

**Indirizzo Email** *(Richiesto)*
- **Nome attributo**: `email`, `emailAddress`, oppure `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/emailaddress`
- **Scopo**: Identificazione univoca dell'utente e notifiche
- **Formato**: Indirizzo email valido

#### Attributi opzionali

**Nome**
- **Nomi attributo**: `firstName`, `givenName`, oppure `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/givenname`
- **Scopo**: Nome visualizzato dall'utente

**Cognome**
- **Nomi attributo**: `lastName`, `surname`, oppure `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/surname`
- **Scopo**: Nome visualizzato dall'utente

**Ruoli** *(Importante per il controllo degli accessi)*
- **Nomi attributo**: `roles`, `groups`, `memberOf`, oppure nomi di attributi personalizzati
- **Scopo**: Assegnazione dei ruoli e permessi in FastComments
- **Formato**: Array di stringhe di ruoli o valori separati da virgole

### Configurazioni comuni degli identity provider

#### Microsoft Azure AD

1. **Aggiungi applicazione aziendale**
   - Cercare "FastComments" o creare un'applicazione SAML personalizzata
   - Utilizzare le informazioni SP fornite da FastComments

2. **Configura attributi**
   - Email: `user.mail` oppure `user.userprincipalname`
   - Nome: `user.givenname`
   - Cognome: `user.surname`
   - Ruoli: `user.assignedroles` oppure gruppi della directory

#### Okta

1. **Crea applicazione SAML**
   - Utilizzare "Create New App" e selezionare SAML 2.0
   - Configurare con le informazioni SP di FastComments

2. **Attribute Statements**
   - Email: `user.email`
   - FirstName: `user.firstName`
   - LastName: `user.lastName`
   - Ruoli: `user.groups` oppure attributi personalizzati

#### Google Workspace

1. **Aggiungi applicazione SAML**
   - Andare su App > App Web e mobile > Aggiungi app > Aggiungi app SAML personalizzata
   - Configurare con le informazioni SP di FastComments

2. **Mappatura degli attributi**
   - Email: Email principale
   - Nome: Nome
   - Cognome: Cognome
   - Ruoli: Gruppi oppure attributi personalizzati

#### Active Directory Federation Services (ADFS)

1. **Aggiungi Relying Party Trust**
   - Utilizzare l'URL dei metadati di FastComments o la configurazione manuale
   - Configurare le informazioni SP come fornite

2. **Regole di attestazione (Claim Rules)**
   - Email: Attestazione Email Address
   - Nome: Attestazione Name ID
   - Ruoli: Appartenenza a gruppi oppure attestazioni personalizzate

### Flessibilità dei nomi degli attributi

FastComments accetta informazioni sui ruoli da più nomi di attributi per adattarsi a diverse configurazioni IdP:

- `roles`
- `groups`
- `memberOf`
- `role`
- `group`
- `http://schemas.microsoft.com/ws/2008/06/identity/claims/role`
- `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/role`

Questa flessibilità garantisce la compatibilità con vari identity provider senza richiedere convenzioni di denominazione degli attributi specifiche.

### Test della configurazione

Dopo aver configurato l'identity provider:

1. Salvare la configurazione dell'IdP
2. Effettuare un test con un account utente di test dedicato
3. Verificare che gli attributi vengano inviati correttamente
4. Controllare che i ruoli siano mappati correttamente
5. Assicurarsi che il flusso di autenticazione si completi con successo

La maggior parte degli identity provider offre strumenti di test SAML per validare la configurazione prima del rilascio agli utenti di produzione.