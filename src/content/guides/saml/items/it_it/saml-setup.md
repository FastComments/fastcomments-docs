Setting up SAML authentication in FastComments requires both configuration in your admin dashboard and setup in your identity provider.

### Prerequisiti

Prima di configurare SAML, assicurati di avere:

- Un piano FastComments Flex o Pro (SAML non è disponibile sul piano Creators)
- Accesso amministrativo al tuo account FastComments
- Accesso amministrativo al tuo identity provider
- I metadati SAML o le informazioni sul certificato del tuo IdP

### Accesso alla configurazione SAML

1. Accedi al tuo [pannello di amministrazione FastComments](https://fastcomments.com/auth/my-account)
2. Vai a **API/SSO Settings** nella barra laterale sinistra
3. Clicca sul pulsante **SAML Config**

Se non vedi il pulsante SAML Config, verifica che:
- Il tuo account abbia il pacchetto richiesto (Flex o Pro)
- Hai le autorizzazioni amministrative
- Il tuo utente abbia i ruoli API Admin o Admin Admin

### Configurazione SAML di base

#### Enable SAML Authentication

1. Seleziona la casella **Enable SAML Authentication**
2. Questo attiva SAML per il tuo tenant e rende disponibili i campi di configurazione

#### Campi obbligatori

**IdP Single Sign-On URL** *(Required)*
- L'URL a cui gli utenti verranno reindirizzati per l'autenticazione
- Di solito fornito dal tuo identity provider
- Esempio: `https://your-company.okta.com/app/fastcomments/sso/saml`

**IdP X.509 Certificate** *(Required)*
- Il certificato pubblico del tuo identity provider
- Utilizzato per verificare l'autenticità delle risposte SAML
- Deve includere il certificato completo con i marcatori BEGIN/END
- Formato di esempio:
```
-----BEGIN CERTIFICATE-----
MIICXjCCAcegAwIBAgIBADANBgkqhkiG9w0BAQsFADA...
-----END CERTIFICATE-----
```

#### Campi opzionali

**IdP Entity ID / Issuer**
- Identifica il tuo identity provider
- Se lasciato vuoto, predefinito all'URL di FastComments
- Dovrebbe corrispondere all'issuer configurato nel tuo IdP

### Configurazione avanzata

#### Impostazioni di sicurezza

**Signature Algorithm**
- Predefinito SHA-256 (consigliato)
- Opzioni: SHA-1, SHA-256, SHA-512
- Dovrebbe corrispondere alla configurazione del tuo IdP

**Digest Algorithm**
- Predefinito SHA-256 (consigliato)
- Utilizzato per il calcolo del digest nelle risposte SAML
- Dovrebbe corrispondere alla configurazione del tuo IdP

**Name ID Format**
- Predefinito il formato Email Address
- Determina come sono formattati gli identificatori utente
- Opzioni comuni: Email Address, Persistent, Transient

#### Crittografia (Opzionale)

**Private Key for Decryption**
- Necessaria solo se il tuo IdP cripta le assertion SAML
- Incolla la tua chiave privata utilizzata per la decrittazione
- La maggior parte delle distribuzioni non richiede la crittografia delle assertion

### Salvataggio della configurazione

1. Controlla tutte le impostazioni per accuratezza
2. Clicca **Save SAML Configuration**
3. Il sistema convaliderà la tua configurazione
4. Se avrà successo, vedrai un messaggio di conferma

### Passi successivi

Dopo aver salvato la configurazione SAML di FastComments:

1. Configura il tuo identity provider utilizzando le informazioni del Service Provider
2. Testa il flusso di autenticazione
3. Configura ruoli e permessi utenti secondo necessità

Le informazioni del Service Provider necessarie per la configurazione del tuo IdP saranno visualizzate una volta che SAML è abilitato.