Quando SAML è abilitato in FastComments, il sistema genera automaticamente le informazioni del Service Provider (SP) che è necessario configurare nel tuo fornitore di identità (IdP).

### Accesso alle informazioni del Service Provider

Le informazioni SP sono visualizzate nella pagina di configurazione SAML dopo l'abilitazione dell'autenticazione SAML. Queste informazioni includono tutti i dettagli di cui il tuo IdP ha bisogno per stabilire la relazione di trust SAML.

### Endpoint del Service Provider

#### SP Entity ID / Audience
**Scopo**: Identifica in modo univoco la tua istanza FastComments come service provider  
**Formato**: `https://fastcomments.com/saml/{your-tenant-id}`  
**Utilizzo**: Configurare questo valore come Entity ID o Audience nel tuo IdP

Questo identificatore garantisce che le risposte SAML siano destinate al tuo specifico tenant FastComments e impedisce che risposte SAML vengano accettate da altre istanze.

#### Assertion Consumer Service (ACS) URL
**Scopo**: L'endpoint dove il tuo IdP invia le risposte SAML dopo l'autenticazione dell'utente  
**Formato**: `https://fastcomments.com/saml/callback/{your-tenant-id}`  
**Utilizzo**: Configurare questo valore come ACS URL o Reply URL nel tuo IdP

Qui gli utenti vengono reindirizzati dopo un'autenticazione riuscita con il tuo fornitore di identità, insieme all'asserzione SAML contenente le informazioni sull'utente.

#### SP Metadata URL
**Scopo**: Fornisce la configurazione SAML completa in formato XML standard  
**Formato**: `https://fastcomments.com/saml/metadata/{your-tenant-id}`  
**Utilizzo**: Alcuni IdP possono importare automaticamente la configurazione usando questo URL

L'URL dei metadata contiene tutte le informazioni necessarie sul SP in formato XML, facilitando la configurazione automatica dei provider di identità compatibili.

#### SAML Login URL
**Scopo**: Link diretto per avviare l'autenticazione SAML per il tuo tenant  
**Formato**: `https://fastcomments.com/saml/login/{your-tenant-id}`  
**Utilizzo**: Collegare gli utenti direttamente all'autenticazione SAML o testare il flusso

Puoi usare questo URL per testare l'autenticazione SAML o fornire agli utenti un link diretto per accedere tramite SAML.

### Supporto dei binding SAML

FastComments supporta i seguenti binding SAML:

#### Binding HTTP-POST
- **Metodo principale**: Binding più comune per le risposte SAML  
- **Sicurezza**: La risposta SAML viene inviata via HTTP POST all'ACS URL  
- **Utilizzo**: Raccomandato per le distribuzioni in produzione

#### Binding HTTP-Redirect
- **Metodo alternativo**: Risposta SAML inviata tramite redirect HTTP  
- **Limitazioni**: Dimensione del payload limitata a causa delle restrizioni della lunghezza dell'URL  
- **Utilizzo**: Supportato ma HTTP-POST è preferito

### Politica del Name ID

FastComments configura la seguente politica Name ID nelle richieste SAML:

- **Formato predefinito**: `urn:oasis:names:tc:SAML:1.1:nameid-format:emailAddress`  
- **Formati alternativi**: Persistent, Transient, Unspecified (configurabili)  
- **Requisito**: L'indirizzo email è usato come identificatore principale dell'utente

### Attributi della richiesta SAML

Quando viene avviata l'autenticazione SAML, FastComments invia richieste con queste caratteristiche:

#### Firma della richiesta
- **Stato**: Opzionale (configurabile)  
- **Algoritmo**: Corrisponde all'algoritmo di firma configurato  
- **Certificato**: Usa il certificato specifico del tenant se la firma della richiesta è abilitata

#### Attributi richiesti
FastComments richiede i seguenti attributi nelle AuthnRequest SAML:

- **Email**: Richiesta per l'identificazione dell'utente  
- **Nome**: Opzionale per scopi di visualizzazione  
- **Cognome**: Opzionale per scopi di visualizzazione  
- **Ruoli/Gruppi**: Opzionale per controllo degli accessi e permessi

### Copiare le informazioni SP

La pagina di configurazione SAML fornisce campi cliccabili che copiano automaticamente le informazioni SP negli appunti:

1. Clicca su qualsiasi campo di informazione SP (Entity ID, ACS URL, ecc.)  
2. Il valore viene copiato automaticamente negli appunti  
3. Incolla il valore nella configurazione del tuo fornitore di identità  
4. Una breve evidenziazione indica la copia avvenuta con successo

Questo rende semplice trasferire con precisione le informazioni SP al tuo IdP senza errori di digitazione.

### Informazioni sul certificato SP

#### Utilizzo del certificato
- **Scopo**: Cripta le comunicazioni e verifica l'identità del SP  
- **Rotazione**: I certificati sono gestiti automaticamente da FastComments  
- **Accesso**: I certificati pubblici sono disponibili tramite l'URL dei metadata

#### Dettagli del certificato
- **Algoritmo**: RSA-2048 o superiore  
- **Validità**: I certificati sono rinnovati automaticamente prima della scadenza  
- **Distribuzione**: Disponibili tramite i metadata SAML standard

### Risoluzione dei problemi della configurazione SP

Se il tuo fornitore di identità segnala problemi con le informazioni SP:

1. **Verifica gli URL**: Assicurati che tutti gli URL utilizzino HTTPS e includano il corretto tenant ID  
2. **Controlla i metadata**: Usa l'URL dei metadata per verificare la configurazione  
3. **Verifica la connettività**: Assicurati che il tuo IdP possa raggiungere gli endpoint di FastComments  
4. **Valida il formato**: Conferma che il tuo IdP supporti il formato delle informazioni SP

I problemi comuni includono:
- Tenant ID errato negli URL  
- Problemi di connettività di rete tra IdP e FastComments  
- IdP che si aspetta formati URL diversi o opzioni di configurazione aggiuntive