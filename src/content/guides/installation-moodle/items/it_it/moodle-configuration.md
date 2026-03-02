La pagina delle impostazioni del plugin si trova in **Amministrazione del sito > Plugin > Plugin locali > FastComments**. Le opzioni disponibili sono:

#### ID del Tenant

Il tuo Tenant ID di FastComments. Trovalo nel <a href="https://fastcomments.com/auth/my-account" target="_blank">dashboard di FastComments</a> nelle impostazioni del tuo account.

#### Segreto API

La tua chiave Segreto API, necessaria per la modalità SSO sicura. Trovala in <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">Il mio account > Segreto API</a>.

#### Modalità SSO

Scegli come vengono autenticati gli utenti. Vedi la sezione [Modalità SSO](#moodle-sso-modes) per i dettagli su ogni opzione.

- **Secure** (consigliato) - autenticazione firmata HMAC-SHA256 lato server
- **Simple** - dati utente lato client senza firma
- **None** - commenti anonimi, nessuna integrazione con il login di Moodle

#### Contesti Pagina

Determina dove appaiono i commenti:

- **Pagine del corso** - commenti nelle pagine principali del corso
- **Pagine di modulo/attività** - commenti sulle singole attività e risorse
- **Entrambe** - commenti su tutti i tipi di pagina

#### Stile dei Commenti

Scegli l'esperienza di commento. Vedi [Stili di commento](#moodle-commenting-styles) per gli screenshot di ogni modalità.

- **Comments** - widget standard di commenti annidati sotto il contenuto della pagina
- **Collab Chat** - discussioni inline tramite selezione del testo con indicatori di presenza
- **Both** - commenti e collab chat attivi insieme

#### URL CDN

L'URL del CDN di FastComments. Predefinito: `https://cdn.fastcomments.com`. Cambia questo con l'URL del CDN dell'UE se i tuoi dati sono ospitati nella regione UE.