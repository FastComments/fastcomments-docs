### L'API di FastComments

FastComments fornisce un'API per interagire con molte risorse. Crea integrazioni con la nostra piattaforma, o anche i tuoi client!

In questa documentazione troverai tutte le risorse supportate dall'API documentate con i rispettivi tipi di richiesta e risposta.

Per i clienti Enterprise, tutto l'accesso all'API viene registrato nel registro di audit.

### SDK generati

FastComments ora genera una [Specifiche API](https://fastcomments.com/js/swagger.json) dal nostro codice (non è ancora completa, ma include molte API).

Disponiamo inoltre di SDK per i linguaggi più diffusi:

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

L'API viene autenticata passando la tua [api key](https://fastcomments.com/auth/my-account/api-secret) come intestazione `X-API-KEY` o come parametro di query `API_KEY`. Avrai inoltre bisogno del tuo `tenantId` per effettuare chiamate all'API. Questo può essere recuperato dalla stessa pagina della tua api key.

### Nota sulla sicurezza

Queste route sono pensate per essere chiamate da un **server**. __NON__ chiamarle da un browser. In questo modo esporrai la tua API key - ciò fornirà pieno accesso al tuo account a chiunque possa visualizzare il codice sorgente di una pagina!

#### Opzione 1 - Intestazioni

- Intestazione: `X-API-KEY`
- Intestazione: `X-TENANT-ID`

#### Opzione 2 - Parametri di query

- Parametro di query: `API_KEY`
- Parametro di query: `tenantId`

### Lettura delle proprie scritture

FastComments fornisce disponibilità Active-Active. Le richieste dal tuo datacenter vengono instradate al [punto di presenza più vicino](https://sophon.fastcomments.com/) al tuo. Questo è automatico, e normalmente puoi osservare la semantica read-your-write. Se vuoi essere sicuro di leggere le tue scritture, puoi bloccare le tue richieste su una certa regione usando quella regione come host API (tuttavia questo di solito non è necessario per la maggior parte delle integrazioni):

- gdc-oregon.fastcomments.com
- gdc-virginia.fastcomments.com
- gdc-singapore.fastcomments.com
- gdc-falkenstein2.fastcomments.com
- gdc-sao-paulo.fastcomments.com
- eudc-helsinki2.fastcomments.com
- eudc-limburg.fastcomments.com
- eudc-france.fastcomments.com

Nota che se fai questo potrebbe essere opportuno definire un fallback, poiché in passato abbiamo deprecato nodi di entrypoint e usato nuovi nomi per lo switchover.