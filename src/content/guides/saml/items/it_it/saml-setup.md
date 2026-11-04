La configurazione dell'autenticazione SAML in FastComments richiede sia la configurazione nella tua dashboard di amministrazione sia l'impostazione nel tuo identity provider.

### Prerequisiti

Prima di configurare SAML, assicurati di avere:

- Un piano FastComments Flex o Pro (SAML non è disponibile sul piano Creators)
- Accesso amministrativo al tuo account FastComments
- Accesso amministrativo al tuo identity provider
- I metadati SAML o le informazioni sul certificato del tuo IdP

### Accesso alla configurazione SAML

1. Accedi alla tua [dashboard di amministrazione FastComments](https://fastcomments.com/auth/my-account)
2. Vai su **API/SSO Settings** nella barra laterale sinistra
3. Fai clic sul pulsante **SAML Config**

Se non vedi il pulsante SAML Config, verifica che:
- Il tuo account abbia il pacchetto richiesto (Flex o Pro)
- Tu abbia permessi amministrativi
- Il tuo utente abbia i ruoli API Admin o Admin Admin

### Configurazione SAML di base

#### Abilitare l'autenticazione SAML

1. Spunta la casella **Enable SAML Authentication**
2. Questo attiva SAML per il tuo tenant e rende disponibili i campi di configurazione

#### Campi obbligatori

**IdP Single Sign-On URL** *(Obbligatorio)*
- L'URL a cui gli utenti verranno reindirizzati per l'autenticazione
- Di solito fornito dal tuo identity provider
- Esempio: `https://your-company.okta.com/app/fastcomments/sso/saml`

**IdP X.509 Certificate** *(Obbligatorio)*
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
- Se lasciato vuoto, viene impostato di default sul tuo URL FastComments
- Deve corrispondere all'issuer configurato nel tuo IdP

### Configurazione avanzata

#### Impostazioni di sicurezza

**Signature Algorithm**
- Predefinito: SHA-256 (consigliato)
- Opzioni: SHA-1, SHA-256, SHA-512
- Deve corrispondere alla configurazione del tuo IdP

**Digest Algorithm**
- Predefinito: SHA-256 (consigliato)
- Utilizzato per il calcolo del digest nelle risposte SAML
- Deve corrispondere alla configurazione del tuo IdP

**Name ID Format**
- Predefinito: formato Email Address
- Determina come vengono formattati gli identificatori utente
- Opzioni comuni: Email Address, Persistent, Transient

#### Crittografia (opzionale)

**Private Key for Decryption**
- Necessaria solo se il tuo IdP crittografa le asserzioni SAML
- Incolla la tua chiave privata utilizzata per la decrittazione
- La maggior parte delle distribuzioni non richiede la crittografia delle asserzioni

### Salvataggio della configurazione

1. Controlla tutte le impostazioni per verificarne l'accuratezza
2. Fai clic su **Save SAML Configuration**
3. Il sistema validerà la tua configurazione
4. Se l'operazione riesce, vedrai un messaggio di conferma

### Prossimi passi

Dopo aver salvato la configurazione SAML di FastComments:

1. Configura il tuo identity provider utilizzando le informazioni del Service Provider
2. Testa il flusso di autenticazione
3. Configura ruoli e permessi utente in base alle necessità

Le informazioni del Service Provider necessarie per la configurazione del tuo IdP verranno visualizzate una volta abilitato SAML.