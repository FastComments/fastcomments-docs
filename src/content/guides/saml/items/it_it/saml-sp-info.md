Quando SAML è abilitato in FastComments, il sistema genera automaticamente le informazioni del Service Provider (SP) che devi configurare nel tuo identity provider.

### Accesso alle Informazioni del Service Provider

Le informazioni SP vengono visualizzate nella tua pagina di configurazione SAML dopo aver abilitato l'autenticazione SAML. Queste informazioni includono tutti i dettagli di cui il tuo identity provider ha bisogno per stabilire la relazione di trust SAML.

### Endpoint del Service Provider

#### SP Entity ID / Audience
**Scopo**: Identifica in modo univoco la tua istanza FastComments come service provider
**Formato**: `https://fastcomments.com/saml/{your-tenant-id}`
**Utilizzo**: Configura questo come Entity ID o Audience nel tuo IdP

Questo identificatore garantisce che le risposte SAML siano destinate al tuo specifico tenant FastComments e impedisce che le risposte SAML vengano accettate da altre istanze.

#### Assertion Consumer Service (ACS) URL
**Scopo**: L'endpoint a cui il tuo IdP invia le risposte SAML dopo l'autenticazione dell'utente
**Formato**: `https://fastcomments.com/saml/callback/{your-tenant-id}`
**Utilizzo**: Configura questo come ACS URL o Reply URL nel tuo IdP

Qui gli utenti vengono reindirizzati dopo un'autenticazione riuscita con il tuo identity provider, insieme all'asserzione SAML contenente le informazioni dell'utente.

#### SP Metadata URL
**Scopo**: Fornisce la configurazione SAML completa in formato XML standard
**Formato**: `https://fastcomments.com/saml/metadata/{your-tenant-id}`
**Utilizzo**: Alcuni IdP possono importare automaticamente la configurazione utilizzando questo URL

L'URL dei metadati contiene tutte le informazioni SP necessarie in formato XML, rendendo facile configurare automaticamente gli identity provider compatibili.

#### SAML Login URL
**Scopo**: Link diretto per avviare l'autenticazione SAML per il tuo tenant
**Formato**: `https://fastcomments.com/saml/login/{your-tenant-id}`
**Utilizzo**: Collega gli utenti direttamente all'autenticazione SAML o testa il flusso

Puoi utilizzare questo URL per testare l'autenticazione SAML o fornire agli utenti un link diretto per accedere tramite SAML.

### Supporto dei Binding SAML

FastComments supporta i seguenti binding SAML:

#### Binding HTTP-POST
- **Metodo Primario**: Il binding più comune per le risposte SAML
- **Sicurezza**: La risposta SAML viene inviata tramite HTTP POST all'ACS URL
- **Utilizzo**: Consigliato per distribuzioni in produzione

#### Binding HTTP-Redirect
- **Metodo Alternativo**: Risposta SAML inviata tramite redirect HTTP
- **Limitazioni**: Dimensione del payload limitata a causa delle restrizioni sulla lunghezza dell'URL
- **Utilizzo**: Supportato, ma HTTP-POST è preferito

### Name ID Policy

FastComments configura la seguente Name ID policy nelle richieste SAML:

- **Formato Predefinito**: `urn:oasis:names:tc:SAML:1.1:nameid-format:emailAddress`
- **Formati Alternativi**: Persistent, Transient, Unspecified (configurabile)
- **Requisito**: L'indirizzo email viene utilizzato come identificatore primario dell'utente

### Attributi delle Richieste SAML

Quando avvia l'autenticazione SAML, FastComments invia richieste con queste caratteristiche:

#### Firma della Richiesta
- **Stato**: Opzionale (configurabile)
- **Algoritmo**: Corrisponde all'algoritmo di firma configurato
- **Certificato**: Utilizza il certificato specifico del tenant se la firma delle richieste è abilitata

#### Attributi Richiesti
FastComments richiede i seguenti attributi nelle SAML AuthnRequests:

- **Email**: Richiesta per l'identificazione dell'utente
- **Nome**: Opzionale per scopi di visualizzazione
- **Cognome**: Opzionale per scopi di visualizzazione
- **Ruoli/Gruppi**: Opzionale per il controllo degli accessi e le autorizzazioni

### Copia delle Informazioni SP

La pagina di configurazione SAML fornisce campi cliccabili che copiano automaticamente le informazioni SP negli appunti:

1. Fai clic su qualsiasi campo di informazioni SP (Entity ID, ACS URL, ecc.)
2. Il valore viene copiato automaticamente negli appunti
3. Incolla il valore nella configurazione del tuo identity provider
4. Una breve evidenziazione indica la copia riuscita

Questo rende facile trasferire con precisione le informazioni SP al tuo IdP senza errori di digitazione.

### Informazioni sul Certificato SP

#### Utilizzo del Certificato
- **Scopo**: Cripta le comunicazioni e verifica l'identità dello SP
- **Rotazione**: I certificati sono gestiti automaticamente da FastComments
- **Accesso**: I certificati pubblici sono disponibili tramite l'URL dei metadati

#### Dettagli del Certificato
- **Algoritmo**: RSA-2048 o superiore
- **Validità**: I certificati vengono rinnovati automaticamente prima della scadenza
- **Distribuzione**: Disponibili tramite i metadati SAML standard

### Risoluzione dei Problemi di Configurazione SP

Se il tuo identity provider segnala problemi con le informazioni SP:

1. **Verifica gli URL**: Assicurati che tutti gli URL utilizzino HTTPS e includano il tenant ID corretto
2. **Controlla i Metadati**: Utilizza l'URL dei metadati per verificare la configurazione
3. **Testa la Connettività**: Assicurati che il tuo IdP possa raggiungere gli endpoint FastComments
4. **Valida il Formato**: Conferma che il tuo IdP supporti il formato delle informazioni SP

I problemi comuni includono:
- Tenant ID errato negli URL
- Problemi di connettività di rete tra IdP e FastComments
- IdP che si aspetta formati URL diversi o opzioni di configurazione aggiuntive