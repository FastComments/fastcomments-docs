La pagina delle impostazioni del plugin si trova in **Amministrazione del sito > Plugin > Plugin locali > FastComments**. Le opzioni disponibili sono:

#### Tenant ID

Il tuo FastComments Tenant ID. Trovalo nella <a href="https://fastcomments.com/auth/my-account" target="_blank">FastComments dashboard</a> nelle impostazioni del tuo account.

#### API Secret

La tua chiave API Secret, richiesta per la modalità SSO Secure. Trovala su <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">My Account > API Secret</a>.

#### SSO Mode

Scegli come gli utenti vengono autenticati. Vedi la sezione [SSO Modes](#items-moodle-sso-modes) per i dettagli su ciascuna opzione.

- **Secure** (consigliato) - autenticazione firmata HMAC-SHA256 lato server
- **Simple** - dati utente lato client senza firma
- **None** - commenti anonimi, nessuna integrazione con il login di Moodle

#### Page Contexts

Controlla dove appaiono i commenti:

- **Course pages** - commenti sulle pagine principali del corso
- **Module/activity pages** - commenti sulle singole attività e risorse
- **Both** - commenti su tutti i tipi di pagina

#### Commenting Style

Scegli l'esperienza di commento. Vedi [Commenting Styles](#items-moodle-commenting-styles) per gli screenshot di ogni modalità.

- **Comments** - widget standard di commenti annidati sotto il contenuto della pagina
- **Collab Chat** - discussioni inline selezionando testo con indicatori di presenza
- **Both** - commenti e collab chat attivi insieme

#### CDN URL

L'URL CDN di FastComments. Per impostazione predefinita è `https://cdn.fastcomments.com`. Cambialo con l'URL CDN per l'UE se i tuoi dati sono ospitati nella regione UE.