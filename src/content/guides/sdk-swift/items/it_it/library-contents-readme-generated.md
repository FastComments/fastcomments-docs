Il FastComments Swift SDK è composto da diversi moduli:

- **Modulo Client** - Client API generato automaticamente per le REST API di FastComments
  - Definizioni complete dei tipi per tutti i modelli API
  - Endpoint sia autenticati (`DefaultAPI`) che pubblici (`PublicAPI`)
  - Supporto completo per async/await
  - Vedi [client/README.md](https://github.com/FastComments/fastcomments-swift/blob/main/client/README.md) per la documentazione dettagliata delle API

- **Modulo SSO** - Utilità Single Sign-On lato server
  - Generazione sicura dei token per l'autenticazione degli utenti
  - Supporto sia per modalità SSO semplici che sicure
  - Firma dei token basata su HMAC-SHA256 utilizzando CryptoKit