L'integrazione LTI 1.3 di FastComments segue il principio dei privilegi minimi: utilizza solo i claim di lancio necessari per identificare l'utente, associare i commenti al corso e alla risorsa corretti e applicare i permessi basati sui ruoli.

Il resto di questa pagina descrive ogni claim consumato dall'integrazione, ogni servizio LTI Advantage che non richiede e ogni categoria di dati che non raccoglie. I revisori di sicurezza e procurement possono ricavare le risposte direttamente dalle tabelle seguenti.

## Elementi di Dati Ricevuti dall'LMS

Ogni lancio LTI 1.3 contiene un JWT firmato dall'LMS. FastComments estrae dal JWT i seguenti claim e non utilizza nulla altro:

| Campo | Claim LTI | Scopo | Obbligatorio | Memorizzato |
|-------|-----------|---------|----------|--------|
| Identificatore utente | `sub` | Identifica l'utente in modo coerente tra i lanci, così che la stessa persona corrisponda allo stesso utente SSO FastComments | Sì | Sì, come parte di un ID SSO interno stabile |
| Nome visualizzato | `name` | Attribuzione mostrata accanto ai commenti dell'utente | Sì (con fallback a "LMS User" se assente) | Sì |
| Email | `email` | Corrispondenza account, notifiche, moderazione, corrispondenza con il supporto | Opzionale (l'integrazione funziona anche senza) | Sì, se fornita |
| URL avatar | `picture` | Visualizzato sui commenti dell'utente | Opzionale | Solo l'URL; FastComments non scarica né rihosta l'immagine |
| Ruoli | `https://purl.imsglobal.org/spec/lti/claim/roles` | Determina se l'utente è amministratore, docente (moderatore) o studente | Sì | Flag derivati `isAdmin` / `isModerator` sulla sessione SSO |
| Contesto corso | `https://purl.imsglobal.org/spec/lti/claim/context` (`id`, `title`) | Associa il thread di commenti al corso LMS corretto | Sì | Sì, come parte dell'identificatore di pagina risolto |
| Resource link | `https://purl.imsglobal.org/spec/lti/claim/resource_link` (`id`) | Associa i commenti all'attività o al posizionamento dello strumento corretto all'interno del corso | Sì, se presente | Sì, come parte dell'identificatore di pagina risolto |
| Deployment ID | `https://purl.imsglobal.org/spec/lti/claim/deployment_id` | Instrada il lancio alla configurazione tenant FastComments corretta | Sì | Sì, sul record di configurazione LTI di FastComments |

## Claim e Scope Dichiarati alla Registrazione

Durante la Dynamic Registration LTI 1.3, FastComments si registra con `scope: ""` (nessuno scope OAuth aggiuntivo) e dichiara solo questi claim OpenID Connect:

`iss`, `sub`, `name`, `email`, `picture`

Registra due tipi di messaggio:

- `LtiResourceLinkRequest` - il lancio standard del corso in FastComments.
- `LtiDeepLinkingRequest` - consente ai docenti di inserire lo strumento FastComments all'interno di un corso.

Non vengono richiesti token di accesso aggiuntivi all'LMS.

## Servizi LTI Advantage Non Richiesti

| Servizio / scope | Richiesto? | Motivo |
|------------------|------------|--------|
| Names and Role Provisioning Services (NRPS) | No | L'integrazione non necessita dell'elenco dei partecipanti al corso; l'identità dell'utente arriva con ogni lancio |
| Assignment and Grade Services (AGS) - scope lineitem, score, result | No | L'integrazione non è consapevole del registro dei voti |
| Deep Linking oltre il ritorno standard di posizionamento | Nessun dato aggiuntivo | Il deep linking viene utilizzato solo per il posizionamento dello strumento da parte dei docenti; nessun contenuto del corso viene enumerato |

## Dati Non Raccolti

Oltre a LTI stesso, FastComments non richiede né riceve dall'LMS o dall'utente quanto segue:

| Categoria | Raccolta? |
|----------|------------|
| Voti degli studenti | No |
| Consegne di compiti | No |
| Registri di presenza | No |
| Elenchi completi dei partecipanti ai corsi | No |
| Identificatori governativi | No |
| Data di nascita | No |
| Indirizzo postale o numero di telefono | No |
| Informazioni finanziarie | No |
| Credenziali amministratore dell'LMS | No |

## Limiti di Accesso

- FastComments riceve dati solo all'interno di un lancio LTI 1.3 autorizzato e firmato dalle chiavi registrate dell'LMS. L'integrazione non richiama l'LMS per ottenere informazioni aggiuntive.
- I token di lancio sono monouso e di breve durata. I token riprodotti o scaduti vengono rifiutati.
- Gli amministratori dell'LMS controllano dove lo strumento viene distribuito all'interno della loro piattaforma. D2L Brightspace, ad esempio, supporta lo scoping per org-unit per deployment e impostazioni di sicurezza per deployment, il che consente agli amministratori di limitare lo strumento a corsi o org unit specifici invece di renderlo disponibile globalmente. Moodle, Blackboard, Sakai e Schoology offrono controlli equivalenti per deployment nelle loro implementazioni LTI 1.3.

## Conservazione e Archiviazione

FastComments conserva i dati derivati da LTI per la durata del servizio di commenti attivo e secondo le impostazioni di conservazione configurate dal cliente. I dati dei commenti sono archiviati in storage di produzione crittografati a riposo. In caso di chiusura dell'account o di richiesta scritta di eliminazione, FastComments elimina o anonimizza i dati del cliente secondo l'accordo applicabile.

Per i dettagli completi su archiviazione e gestione dei dati, consultare la <a href="https://fastcomments.com/privacy-policy" target="_blank">Informativa sulla Privacy di FastComments</a>.

## Frequenza di Revisione

Qualsiasi nuova funzionalità LTI che richiederebbe claim, scope o servizi LTI Advantage aggiuntivi viene esaminata prima del rilascio per confermare che l'accesso richiesto sia necessario e proporzionato alla funzionalità che viene rilasciata.

## Dichiarazione Breve per i Questionari di Sicurezza

> FastComments applica il principio dei privilegi minimi e la minimizzazione dei dati alla sua integrazione LTI 1.3. L'integrazione utilizza solo i claim di lancio LTI necessari per autenticare l'utente (`sub`, `name`, `email`, `picture`), determinarne il ruolo e identificare il corso e la risorsa a cui appartengono i commenti. FastComments non richiede Names and Role Provisioning Services, Assignment and Grade Services, dati del registro dei voti, presenza, elenchi completi dei partecipanti o accesso amministrativo all'LMS. Gli amministratori dell'LMS mantengono il controllo su quali org unit, corsi e deployment lo strumento è disponibile.