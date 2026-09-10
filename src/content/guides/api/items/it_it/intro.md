### L'API FastComments

FastComments fornisce un'API per interagire con molte risorse. Crea integrazioni con la nostra piattaforma, o addirittura crea i tuoi client!

In questa documentazione, troverai tutte le risorse supportate dall'API documentate con i loro tipi di richiesta e risposta.

Per i clienti Enterprise, tutti gli accessi all'API sono registrati nel Registro di Audit.

### SDK Generati

FastComments ora genera una [API Spec](https://fastcomments.com/js/swagger.json) dal nostro codice (non è ancora completa, ma include molte API).

Abbiamo anche ora SDK per i linguaggi più popolari:

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

L'API è autenticata passando la tua [api key](https://fastcomments.com/auth/my-account/api-secret) come header `X-API-KEY` o come parametro di query `API_KEY`. Avrai anche bisogno del tuo `tenantId` per effettuare chiamate API. Questo può essere recuperato dalla stessa pagina della tua chiave API.

### Nota di Sicurezza

Queste rotte sono destinate a essere chiamate da un **server**. __NON__ chiamarle da un browser. Farlo esporrà la tua chiave API – questo darà pieno accesso al tuo account a chiunque possa visualizzare il codice sorgente di una pagina!

#### Opzione di Autenticazione Uno - Header

- Header: `X-API-KEY`
- Header: `X-TENANT-ID`

#### Opzione di Autenticazione Due - Parametri di Query

- Query Param: `API_KEY`
- Query Param: `tenantId`

#### Opzione di Autenticazione Tre - Token Bearer OAuth

- Header: `Authorization: Bearer fcat_...`

Le applicazioni che si connettono tramite il [MCP server](https://docs.fastcomments.com/guide-llm-kit.html) ottengono un token tramite OAuth invece di una chiave API. Quel token funziona su ogni endpoint qui. Il tenant è implicito nel token, quindi `tenantId` è opzionale, ma deve corrispondere al token quando fornito. Le richieste `GET` richiedono lo scope `read` e ogni altro metodo richiede lo scope `write`. La scoperta inizia a `https://fastcomments.com/.well-known/oauth-authorization-server`.

### Leggere le proprie scritture

FastComments fornisce disponibilità Active‑Active. Le richieste dal tuo data center sono instradate al [punto di presenza più vicino](https://sophon.fastcomments.com/) al tuo. Questo è automatico e, normalmente, puoi osservare la semantica di lettura‑scrittura. Se vuoi essere sicuro di leggere le tue proprie scritture, puoi fissare le tue richieste a una certa regione usando quella regione come host API (tuttavia, di solito non è necessario per la maggior parte delle integrazioni):

- gdc-oregon.fastcomments.com
- gdc-virginia.fastcomments.com
- gdc-singapore.fastcomments.com
- gdc-falkenstein2.fastcomments.com
- gdc-sao-paulo.fastcomments.com
- eudc-helsinki2.fastcomments.com
- eudc-limburg.fastcomments.com
- eudc-france.fastcomments.com

Nota che se fai questo potresti voler definire un fallback, poiché in passato abbiamo deprecato nodi di ingresso e utilizziamo nuovi nomi per il passaggio.