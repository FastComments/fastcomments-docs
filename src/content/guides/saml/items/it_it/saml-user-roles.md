FastComments mappa i ruoli utente SAML alle autorizzazioni interne, abilitando il controllo degli accessi basato sui ruoli per la tua organizzazione.

### Sistema di Ruoli di FastComments

FastComments utilizza un sistema di autorizzazioni basato sui ruoli in cui gli utenti possono avere uno o più ruoli che determinano i loro livelli di accesso e le loro capacità.

### Ruoli FastComments disponibili

#### Ruoli amministrativi

**fc-account-owner**
- **Permissions**: Accesso amministrativo completo
- **Capabilities**: Tutte le funzionalità, gestione fatturazione, gestione utenti
- **Use Case**: Amministratori principali dell'account e proprietari

**fc-admin-admin**  
- **Permissions**: Accesso amministrativo alla maggior parte delle funzionalità
- **Capabilities**: Gestione utenti, configurazione, moderazione. **Può amministrare altri amministratori.**
- **Use Case**: Amministratori secondari e personale IT

**fc-billing-admin**
- **Permissions**: Gestione fatturazione e abbonamenti
- **Capabilities**: Metodi di pagamento, fatture, modifiche all'abbonamento
- **Use Case**: Membri del team finanziario e contatti per la fatturazione

#### Ruoli specializzati

**fc-analytics-admin**
- **Permissions**: Accesso ad analytics e reportistica
- **Capabilities**: Visualizzare statistiche del sito, dati di engagement degli utenti
- **Use Case**: Team marketing e analisti dei dati

**fc-api-admin**
- **Permissions**: Accesso e gestione API
- **Capabilities**: Credenziali API, configurazione webhook
- **Use Case**: Sviluppatori e integratori tecnici

**fc-moderator**
- **Permissions**: Capacità di moderazione dei commenti
- **Capabilities**: Approvare/rifiutare commenti, gestire spam
- **Use Case**: Moderatori della community e gestori dei contenuti

### Configurazione dei ruoli del provider di identità

#### Origini degli attributi SAML

FastComments accetta informazioni sui ruoli da vari nomi di attributi SAML per garantire la compatibilità con diversi provider di identità:

**Standard Attribute Names**:
- `roles`
- `groups` 
- `memberOf`
- `role`
- `group`

**Microsoft/ADFS Attributes**:
- `http://schemas.microsoft.com/ws/2008/06/identity/claims/role`
- `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/role`

#### Formati dei ruoli supportati

**Array Format** *(Preferito)*:
```xml
<saml:Attribute Name="roles">
    <saml:AttributeValue>fc-admin-admin</saml:AttributeValue>
    <saml:AttributeValue>fc-moderator</saml:AttributeValue>
</saml:Attribute>
```

**Comma-Separated Format**:
```xml
<saml:Attribute Name="roles">
    <saml:AttributeValue>fc-admin-admin,fc-moderator</saml:AttributeValue>
</saml:Attribute>
```

**Single Role Format**:
```xml
<saml:Attribute Name="roles">
    <saml:AttributeValue>fc-admin-admin</saml:AttributeValue>
</saml:Attribute>
```

### Configurazione dei ruoli nel provider di identità

#### Microsoft Azure AD

1. **App Roles Configuration**:
   - Definire i ruoli FastComments nella tua applicazione Azure AD
   - Assegnare agli utenti i ruoli dell'app appropriati
   - Configurare le claim per includere i ruoli assegnati

2. **Attribute Mapping**:
   ```
   Attribute Name: roles
   Source Attribute: user.assignedroles
   ```

#### Okta

1. **Group Assignment**:
   - Creare gruppi che corrispondano ai nomi dei ruoli FastComments
   - Assegnare gli utenti ai gruppi appropriati
   - Configurare le attribute statements

2. **Attribute Statement**:
   ```
   Name: roles
   Value: user.groups
   Filter: Starts with "fc-"
   ```

#### Google Workspace

1. **Group Mapping**:
   - Creare unità organizzative o gruppi
   - Nominare i gruppi con i prefissi dei ruoli FastComments
   - Configurare il mapping degli attributi

2. **Custom Attributes**:
   ```
   Attribute Name: roles
   Value: Groups or custom schema attribute
   ```

### Comportamento predefinito dell'utente

#### Utenti senza ruoli

Quando un utente SAML non ha ruoli o ha ruoli non riconosciuti:
- L'utente viene creato come commentatore standard
- Non viene concesso accesso amministrativo
- Può pubblicare e gestire i propri commenti
- Non può accedere alle funzionalità della dashboard di amministrazione

#### Eredità dei ruoli

- Gli utenti possono avere più ruoli contemporaneamente
- Le autorizzazioni sono cumulative (si applica il livello di autorizzazione più elevato)
- Le modifiche ai ruoli nel provider di identità (IdP) sono riflesse al prossimo accesso

### Gestione degli utenti SAML

#### Creazione utente

Quando un utente effettua il login tramite SAML per la prima volta:
1. **User Account**: Creato automaticamente con l'email come identificatore
2. **Role Assignment**: I ruoli vengono applicati in base agli attributi SAML
3. **Profile Information**: Nome/cognome popolati se forniti
4. **Permission Activation**: I ruoli diventano attivi immediatamente

#### Aggiornamenti dei ruoli

Gli utenti SAML esistenti ricevono aggiornamenti dei ruoli:
1. **Login Trigger**: Gli aggiornamenti dei ruoli avvengono durante ogni login SAML
2. **Immediate Effect**: Le nuove autorizzazioni si applicano immediatamente
3. **Role Removal**: I ruoli rimossi vengono revocati automaticamente
4. **Audit Trail**: Le modifiche ai ruoli vengono registrate nei log di audit

### Mappatura dei ruoli personalizzata

#### Personalizzazione aziendale

Per i clienti enterprise con requisiti specifici:
- I nomi dei ruoli personalizzati possono essere mappati alle autorizzazioni di FastComments
- Possono essere implementate gerarchie complesse di ruoli
- Possono essere configurati controlli di accesso specifici per dipartimento

Contatta il supporto FastComments per configurazioni di mappatura dei ruoli personalizzate.

#### Validazione dei ruoli

FastComments valida i ruoli in ingresso:
- I ruoli non riconosciuti vengono ignorati (non rifiutati)
- Gli attributi di ruolo malformati vengono registrati per il troubleshooting
- Gli utenti mantengono i ruoli esistenti se l'asserzione SAML non contiene informazioni sui ruoli

### Migliori pratiche

#### Gestione dei ruoli

1. **Principle of Least Privilege**: Assegnare le autorizzazioni minime necessarie
2. **Regular Auditing**: Revisionare periodicamente i ruoli utenti e gli accessi  
3. **Clear Naming**: Usare nomi di gruppo descrittivi nel tuo IdP
4. **Documentation**: Mantenere la documentazione delle assegnazioni dei ruoli

#### Considerazioni sulla sicurezza

1. **Role Attributes**: Assicurarsi che gli attributi dei ruoli siano adeguatamente protetti nelle risposte SAML
2. **Attribute Validation**: Verificare che solo i sistemi autorizzati possano assegnare ruoli
3. **Access Reviews**: Revisionare regolarmente le assegnazioni dei ruoli amministrativi
4. **Monitoring**: Monitorare le modifiche ai ruoli e le azioni amministrative

### Risoluzione dei problemi relativi ai ruoli

#### Problemi comuni

**Roles Not Applied**:
- Verificare che i nomi degli attributi SAML corrispondano ai formati supportati
- Verificare che l'IdP stia inviando le informazioni sui ruoli
- Confermare che i valori dei ruoli corrispondano esattamente ai nomi dei ruoli FastComments

**Access Denied**:
- Verificare che all'utente sia assegnato il ruolo appropriato nell'IdP
- Controllare l'ortografia dei ruoli e la distinzione tra maiuscole e minuscole
- Confermare che il ruolo sia correttamente formattato nella risposta SAML

**Missing Permissions**:
- Riesaminare le definizioni dei ruoli e le autorizzazioni richieste
- Controllare eventuali assegnazioni di ruolo in conflitto
- Verificare che l'utente abbia effettuato il login dopo le modifiche ai ruoli