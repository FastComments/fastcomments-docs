Vai a **Amministrazione > Configurazione > Contenuto > FastComments** (`/admin/config/content/fastcomments`).

### Impostazioni

- **Tenant ID** (obbligatorio) - Il tuo Tenant ID di FastComments. Trovalo in [Impostazioni > API/SSO](https://fastcomments.com/auth/my-account/api) ([UE](https://eu.fastcomments.com/auth/my-account/api)).
- **API Secret** - Richiesto per Secure SSO, la verifica dei webhook e la sincronizzazione delle pagine. Si trova in [Impostazioni > API/SSO](https://fastcomments.com/auth/my-account/api) ([UE](https://eu.fastcomments.com/auth/my-account/api)).
- **Modalità SSO** - Integrazione Single Sign-On:
  - **Nessuna** - Nessuno SSO, gli utenti commentano come ospiti o creano account FastComments.
  - **Semplice** - Trasmette le informazioni utente di Drupal (nome, email, avatar) a FastComments senza verifica lato server.
  - **Sicura** - Usa la verifica HMAC-SHA256 per autenticare in modo sicuro gli utenti Drupal con FastComments (consigliato).
- **Stile dei commenti** - Il tipo di widget da visualizzare:
  - **Live Comments** - Commenti annidati in tempo reale.
  - **Streaming Chat** - Interfaccia di chat in diretta.
  - **Collab Chat** - Annotazione collaborativa tramite selezione del testo sull'area principale del contenuto.
  - **Collab Chat + Comments** - Sia collab chat che commenti standard.
- **URL CDN** - URL CDN di FastComments (predefinito: `https://cdn.fastcomments.com`).
- **URL del sito** - URL del sito FastComments (predefinito: `https://fastcomments.com`).
- **Notifiche email** - Invia un'email agli autori del contenuto quando viene pubblicato un nuovo commento sul loro contenuto.

### Aggiungere commenti ai tipi di contenuto

Aggiungi il campo **FastComments** ai tuoi tipi di contenuto tramite **Struttura > Tipi di contenuto > [type] > Gestisci campi**. Il campo ha un interruttore di stato e un identificatore personalizzato opzionale per ogni entità.

### Residenza dei dati UE

Per la residenza dei dati nell'UE, aggiorna:
- **URL CDN** a `https://cdn-eu.fastcomments.com`
- **URL del sito** a `https://eu.fastcomments.com`