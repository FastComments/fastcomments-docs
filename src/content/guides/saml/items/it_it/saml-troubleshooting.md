Questa guida copre problemi comuni di autenticazione SAML e le loro soluzioni.

### Problemi relativi a certificati e sicurezza

#### Errore di certificato non valido

**Sintomi**:
- Errore "Certificate validation failed"
- Gli utenti non riescono a completare l'autenticazione SAML
- Le risposte SAML vengono rifiutate

**Cause comuni**:
- Il formato del certificato è errato
- Il certificato è scaduto
- È stato fornito il certificato sbagliato
- Caratteri extra o spazi bianchi nel certificato

**Soluzioni**:
1. **Verificare il formato del certificato**:
   - Assicurarsi che il certificato includa i marcatori `-----BEGIN CERTIFICATE-----` e `-----END CERTIFICATE-----`
   - Rimuovere eventuali spazi bianchi o interruzioni di riga extra
   - Copiare il certificato direttamente dai metadata o dalla configurazione dell'IdP

2. **Controllare la validità del certificato**:
   - Verificare che il certificato non sia scaduto
   - Confermare che il certificato sia per l'IdP corretto
   - Usare validatori di certificati online per controllare il formato

3. **Scaricare nuovamente il certificato**:
   - Scaricare un certificato aggiornato dall'IdP
   - Usare l'URL dei metadata dell'IdP se disponibile
   - Confermare che il certificato corrisponda alla configurazione attuale dell'IdP

#### Verifica della firma fallita

**Sintomi**:
- Errori di validazione della firma dell'asserzione SAML
- Autenticazione fallita dopo il login sull'IdP
- Messaggi di errore "Invalid signature"

**Soluzioni**:
1. **Incompatibilità dell'algoritmo**:
   - Controllare che l'algoritmo di firma in FastComments corrisponda a quello dell'IdP
   - Provare diversi algoritmi di firma (SHA-256, SHA-1, SHA-512)
   - Verificare che l'algoritmo di digest corrisponda alla configurazione dell'IdP

2. **Problemi di certificato**:
   - Assicurarsi che il certificato di firma corretto sia configurato
   - Verificare che il certificato corrisponda alla chiave privata usata dall'IdP
   - Controllare eventuali rotazioni di certificato nell'IdP

### Problemi di configurazione

#### Entity ID o ACS URL errati

**Sintomi**:
- L'IdP segnala "Unknown Service Provider"
- Le risposte SAML vengono inviate all'endpoint sbagliato
- L'autenticazione non si completa

**Soluzioni**:
1. **Verificare le informazioni SP**:
   - Copiare esattamente l'Entity ID dalla configurazione FastComments
   - Assicurarsi che l'ACS URL corrisponda al formato: `https://fastcomments.com/saml/callback/{tenant-id}`
   - Controllare eventuali errori di battitura nell'ID tenant

2. **Configurazione IdP**:
   - Aggiornare l'IdP con l'Entity ID SP corretto
   - Configurare l'ACS/Reply URL appropriato
   - Verificare le impostazioni di binding dell'IdP (HTTP-POST preferito)

#### Attributi mancanti o errati

**Sintomi**:
- Utenti creati senza ruoli corretti
- Mancano informazioni del profilo utente
- Errori "Email required"

**Soluzioni**:
1. **Attributo email**:
   - Assicurarsi che l'IdP invii l'attributo email
   - Controllare il mapping del nome dell'attributo (email, emailAddress, ecc.)
   - Verificare che il valore dell'email sia un indirizzo email valido

2. **Attributi di ruolo**:
   - Confermare che l'IdP invii informazioni su ruoli/gruppi
   - Controllare che i nomi degli attributi di ruolo corrispondano alle aspettative di FastComments
   - Verificare che i valori dei ruoli corrispondano esattamente ai nomi dei ruoli in FastComments

3. **Formato degli attributi**:
   - Testare sia il formato array sia quello con ruoli separati da virgola
   - Assicurarsi che i valori degli attributi non abbiano spazi bianchi extra
   - Controllare la sensibilità alle maiuscole dei nomi dei ruoli

### Problemi nel flusso di autenticazione

#### Loop di reindirizzamento

**Sintomi**:
- Il browser reindirizza all'infinito tra FastComments e l'IdP
- L'autenticazione non si completa mai
- Molti reindirizzamenti mostrati negli strumenti di sviluppo del browser

**Soluzioni**:
1. **Controllare la configurazione SP**:
   - Verificare che l'Entity ID corrisponda esattamente alla configurazione dell'IdP
   - Assicurarsi che l'ACS URL sia configurato correttamente nell'IdP
   - Controllare la presenza di slash finali negli URL

2. **Problemi di sessione**:
   - Cancellare i cookie del browser e riprovare
   - Testare in una finestra di navigazione in incognito/privata
   - Controllare le impostazioni di timeout della sessione

#### Accesso negato dopo l'autenticazione

**Sintomi**:
- L'autenticazione SAML riesce
- L'utente viene reindirizzato a FastComments
- Viene mostrato "Access denied" o errore di permessi

**Soluzioni**:
1. **Assegnazione dei ruoli**:
   - Verificare che l'utente abbia ruoli appropriati nell'IdP
   - Controllare che l'attributo ruolo venga inviato nella risposta SAML
   - Confermare che i nomi dei ruoli corrispondano esattamente ai requisiti di FastComments

2. **Limitazioni del pacchetto**:
   - Verificare che l'account abbia il piano Flex o Pro
   - Controllare che la funzionalità SAML sia abilitata per il pacchetto
   - Contattare il supporto se il pacchetto include SAML ma la funzionalità non è disponibile

### Problemi specifici del Identity Provider

#### Microsoft Azure AD

**Problemi comuni**:
- Le assegnazioni di ruolo dell'app non vengono riflesse nei token
- Le claim non vengono inviate correttamente
- Requisiti di assegnazione utente

**Soluzioni**:
- Controllare l'assegnazione utente all'applicazione FastComments
- Verificare che i ruoli dell'app siano configurati correttamente
- Assicurarsi che il mapping delle claim includa gli attributi richiesti

#### Okta

**Problemi comuni**:
- I filtri di gruppo non funzionano correttamente
- Le dichiarazioni di attributo sono configurate in modo errato
- Problemi di assegnazione dell'applicazione

**Soluzioni**:
- Rivedere la configurazione delle dichiarazioni di attributo
- Controllare l'assegnazione dei gruppi e le regole di filtraggio
- Verificare che l'applicazione sia assegnata agli utenti/gruppi appropriati

#### Google Workspace

**Problemi comuni**:
- Gli attributi personalizzati non vengono mappati correttamente
- L'appartenenza ai gruppi non viene inviata
- Errori nella configurazione dell'app SAML

**Soluzioni**:
- Configurare lo schema personalizzato per gli attributi di ruolo
- Controllare la propagazione dell'appartenenza ai gruppi
- Verificare il mapping degli attributi dell'applicazione SAML

### Problemi di rete e connettività

#### Errori di timeout

**Sintomi**:
- Il processo di autenticazione va in timeout
- Errori "Request timeout" o simili
- Flusso di autenticazione lento

**Soluzioni**:
1. **Connettività di rete**:
   - Controllare che le regole del firewall permettano la comunicazione con FastComments
   - Verificare la risoluzione DNS per fastcomments.com
   - Testare la connettività di rete dall'IdP verso FastComments

2. **Problemi di performance**:
   - Controllare i tempi di risposta dell'IdP
   - Verificare che la validazione della catena di certificati non sia lenta
   - Considerare la latenza di rete tra IdP e utenti

#### Problemi SSL/TLS

**Sintomi**:
- Avvisi di certificato durante l'autenticazione
- Fallimenti nella stretta di mano SSL
- Errori "Secure connection failed"

**Soluzioni**:
- Assicurarsi che tutti gli endpoint SAML usino HTTPS
- Controllare la validità dei certificati per tutti i domini coinvolti
- Verificare la compatibilità della versione TLS

### Debugging e logging

#### Abilitare informazioni di debug

1. **Strumenti di sviluppo del browser**:
   - Monitorare la scheda Network durante il flusso SAML
   - Controllare la Console per errori JavaScript
   - Esaminare le richieste POST SAML (se visibili)

2. **Logging dell'IdP**:
   - Abilitare il debug SAML nel tuo IdP
   - Revisionare i log dell'IdP per dettagli su richiesta/risposta SAML
   - Controllare eventuali problemi di mapping degli attributi

#### Messaggi di log comuni

**Log di FastComments**:
- "SAML config not found" - SAML non abilitato o mal configurato
- "Invalid certificate" - La validazione del certificato è fallita
- "Missing email attribute" - L'email richiesta non è fornita nella risposta SAML

**Log dell'IdP**:
- "Unknown service provider" - Mismatch dell'Entity ID
- "Invalid ACS URL" - URL Assertion Consumer Service errato
- "User not assigned" - L'utente non ha accesso all'applicazione SAML

### Ottenere aiuto

#### Informazioni da raccogliere

Quando contatti il supporto, fornisci:
- Messaggi di errore esatti e timestamp
- Dettagli di configurazione SAML (senza dati sensibili)
- Tipo e versione dell'IdP
- Passaggi per riprodurre il problema
- Informazioni su browser e rete

#### Supporto FastComments

Per problemi relativi a SAML:
1. Usa il [support portal](https://fastcomments.com/auth/my-account/help)
2. Includi l'ID tenant e le email degli utenti interessati
3. Fornisci messaggi di errore e dettagli di configurazione
4. Specifica il tipo di IdP e l'approccio di configurazione

#### Supporto IdP

Per problemi specifici dell'IdP:
- Consultare la documentazione dell'IdP per il troubleshooting SAML
- Usare i canali di supporto dell'IdP per problemi di configurazione
- Sfruttare i forum della community dell'IdP per problemi comuni

### Suggerimenti per la prevenzione

#### Best practices

1. **Testare accuratamente**:
   - Testare le modifiche di configurazione in un ambiente non di produzione
   - Verificare con più utenti di test
   - Documentare le configurazioni funzionanti

2. **Monitorare regolarmente**:
   - Configurare il monitoraggio per i fallimenti di autenticazione SAML
   - Revisionare le date di scadenza dei certificati
   - Monitorare eventuali cambiamenti di configurazione dell'IdP

3. **Documentazione**:
   - Mantenere la documentazione della configurazione SAML
   - Documentare eventuali configurazioni personalizzate o soluzioni alternative
   - Tenere aggiornati i contatti degli amministratori dell'IdP

#### Manutenzione proattiva

1. **Gestione dei certificati**:
   - Monitorare le date di scadenza dei certificati
   - Pianificare le procedure di rotazione dei certificati
   - Testare gli aggiornamenti dei certificati prima della scadenza

2. **Revisioni della configurazione**:
   - Revisionare regolarmente la configurazione SAML
   - Verificare che la configurazione dell'IdP rimanga aggiornata
   - Aggiornare la documentazione man mano che vengono effettuate modifiche