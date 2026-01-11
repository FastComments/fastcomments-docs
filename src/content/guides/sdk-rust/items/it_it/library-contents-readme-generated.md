Lo SDK Rust di FastComments è composto da diversi moduli:

- **Client Module** - Client API generato automaticamente per le REST APIs di FastComments
  - Definizioni di tipo complete per tutti i modelli API
  - Endpoint sia autenticati (`DefaultApi`) che pubblici (`PublicApi`)
  - Supporto completo per async/await con tokio
  - Vedi [client/README.md](https://github.com/FastComments/fastcomments-rust/blob/main/client/README.md) per la documentazione dettagliata delle API

- **SSO Module** - Utility Single Sign-On lato server
  - Generazione sicura di token per l'autenticazione degli utenti
  - Supporto per modalità SSO sia semplici che sicure
  - Firma dei token basata su HMAC-SHA256

- **Core Types** - Definizioni di tipo condivise e utility
  - Modelli di commento e strutture di metadata
  - Configurazioni di utenti e tenant
  - Funzioni di aiuto per operazioni comuni