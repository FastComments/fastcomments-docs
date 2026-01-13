SAML (Security Assertion Markup Language) è uno standard aperto basato su XML per lo scambio di dati di autenticazione e autorizzazione tra le parti, 
in particolare tra un provider di identità (IdP) e un provider di servizi (SP).

### Come funziona SAML

SAML consente il single sign-on (SSO) permettendo agli utenti di autenticarsi una sola volta con il proprio provider di identità e poi accedere a più applicazioni 
senza reinserire le credenziali. Quando un utente tenta di accedere a FastComments:

1. **Richiesta di autenticazione**: FastComments reindirizza l'utente al tuo IdP
2. **Autenticazione utente**: L'utente si autentica con il tuo IdP (ad es., Active Directory, Okta, Azure AD)
3. **Risposta SAML**: L'IdP invia a FastComments un'asserzione SAML firmata
4. **Accesso utente**: FastComments valida l'asserzione e concede l'accesso all'utente autenticato

### Vantaggi di SAML

- **Sicurezza migliorata**: L'autenticazione centralizzata riduce i rischi di sicurezza legati alle password
- **Esperienza utente migliorata**: Gli utenti effettuano il login una volta e accedono a più applicazioni senza interruzioni
- **Conformità**: Aiuta a soddisfare i requisiti normativi per il controllo degli accessi e le tracce di audit
- **Controllo amministrativo**: Gli amministratori IT mantengono la gestione centralizzata degli utenti

### Supporto SAML 2.0

FastComments implementa SAML 2.0, la versione dello standard SAML più diffusa. La nostra implementazione supporta:

- binding HTTP-POST e HTTP-Redirect
- risposte e asserzioni SAML firmate
- asserzioni crittografate (opzionale)
- molteplici algoritmi di firma e digest
- vari formati di identificatore di nome