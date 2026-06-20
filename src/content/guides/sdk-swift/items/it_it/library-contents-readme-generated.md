Lo SDK FastComments per Swift è composto da diversi moduli:

- **Client Module** - Client per le API REST di FastComments
  - Definizioni di tipo complete per tutti i modelli API
  - Metodi autenticati (`DefaultAPI`), pubblici (`PublicAPI`) e di moderazione (`ModerationAPI`)
  - Supporto completo per async/await
  - Vedi [client/README.md](https://github.com/FastComments/fastcomments-swift/blob/main/client/README.md) per la documentazione dettagliata delle API

- **SSO Module** - Utility server-side per il Single Sign-On
  - Generazione sicura di token per l'autenticazione degli utenti
  - Supporto per entrambe le modalità SSO: semplice e sicura
  - Firma dei token basata su HMAC-SHA256 tramite CryptoKit