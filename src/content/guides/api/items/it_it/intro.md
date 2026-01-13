### L'API di FastComments

FastComments fornisce un'API per interagire con molte risorse. Crea integrazioni con la nostra piattaforma, o persino costruisci i tuoi client!

In questa documentazione troverai tutte le risorse supportate dall'API documentate con i loro tipi di richiesta e risposta.

Per i clienti Enterprise, tutto l'accesso all'API viene registrato nel registro di audit.

### SDK generati

FastComments ora genera una [Specifiche API](https://fastcomments.com/js/swagger.json) dal nostro codice (non è ancora completa, ma include molte API).

Abbiamo inoltre SDK per i linguaggi più diffusi:

- [fastcomments-cpp](./guide-sdk-cpp.html)
- [fastcomments-go](./guide-sdk-go.html)
- [fastcomments-java](./guide-sdk-java.html)
- [fastcomments-sdk-js](./guide-sdk-javascript.html)
- [fastcomments-nim](./guide-sdk-nim.html)
- [fastcomments-php](guide-sdk-php.html)
- [fastcomments-php-sso](./guide-sdk-php-sso.html)
- [fastcomments-python](./guide-sdk-python.html)
- [fastcomments-ruby](./guide-sdk-ruby.html)
- [fastcomments-rust](./guide-sdk-rust.html)
- [fastcomments-swift](./guide-sdk-swift.html)

### Autenticazione

L'API si autentica passando la tua [chiave API](https://fastcomments.com/auth/my-account/api-secret) come intestazione `X-API-KEY` oppure come parametro di query `API_KEY`. Avrai anche bisogno del tuo `tenantId` per effettuare chiamate API. Puoi recuperarlo dalla stessa pagina della tua chiave API.

### Nota sulla sicurezza

Questi endpoint sono pensati per essere chiamati da un **server**. __NON CHIAMARLI__ da un browser. In questo modo esporrai la tua chiave API - ciò concederà accesso completo al tuo account a chiunque possa visualizzare il codice sorgente di una pagina!

#### Opzione di autenticazione uno - Intestazioni

- Header: `X-API-KEY`
- Header: `X-TENANT-ID`

#### Opzione di autenticazione due - Parametri di query

- Query Param: `API_KEY`
- Query Param: `tenantId`

---