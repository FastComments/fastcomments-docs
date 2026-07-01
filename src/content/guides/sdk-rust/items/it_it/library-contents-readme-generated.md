The FastComments Rust SDK consists of several modules:

- **Client Module** - Client API per le API REST di FastComments
  - Definizioni di tipo complete per tutti i modelli API
  - Tre client API che coprono tutti i metodi di FastComments:
    - `default_api` (**DefaultApi**) - Metodi autenticati con chiave API per uso server-side
    - `public_api` (**PublicApi**) - Metodi pubblici, senza chiave API, sicuri da chiamare da browser e app mobile
    - `moderation_api` (**ModerationApi**) - Una suite completa di API di moderazione in tempo reale e veloci. Ogni metodo di Moderazione accetta un parametro `sso` e può autenticarsi tramite SSO o un cookie di sessione FastComments.com.
  - Supporto completo async/await con tokio
  - Vedi [client/README.md](https://github.com/FastComments/fastcomments-rust/blob/main/client/README.md) per la documentazione dettagliata delle API

- **SSO Module** - Utilità di Single Sign-On lato server
  - Generazione sicura di token per l'autenticazione degli utenti
  - Supporto per entrambi i modi SSO semplici e sicuri
  - Firma di token basata su HMAC-SHA256

- **Core Types** - Definizioni di tipo condivise e utilità
  - Modelli di commento e strutture di metadati
  - Configurazioni di utenti e tenant
  - Funzioni di supporto per operazioni comuni