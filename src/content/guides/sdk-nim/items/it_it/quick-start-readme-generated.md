### Utilizzo delle API Autenticate (DefaultAPI)

**Importante:** Gli endpoint autenticati richiedono che la tua chiave API sia impostata come intestazione `x-api-key`.

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_default
import fastcomments/models/model_comment_data

let client = newHttpClient()
client.headers["x-api-key"] = "your-api-key"

# Effettua chiamate API autenticate.
# I parametri richiesti (e il corpo della richiesta) sono posizionali; i parametri opzionali
# sono passati tramite l'oggetto options dell'operazione.
let (response, httpResponse) = getComments(
  httpClient = client,
  tenantId = "your-tenant-id",
  options = GetCommentsOptions(
    urlId: "your-url-id",
    direction: SortDirections.DESC
  )
)

if response.isSome:
  let resp = response.get()
  if resp.comments.isSome:
    echo "Found ", resp.comments.get().len, " comments"
```

### Utilizzo delle API Pubbliche (PublicAPI)

Gli endpoint pubblici non richiedono autenticazione:

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_public

let client = newHttpClient()

# Effettua chiamate API pubbliche.
# tenantId e urlId sono obbligatori (posizionali); tutto il resto è opzionale.
let (response, httpResponse) = getCommentsPublic(
  httpClient = client,
  tenantId = "your-tenant-id",
  urlId = "your-url-id",
  options = GetCommentsPublicOptions(
    direction: SortDirections.DESC
  )
)

if response.isSome:
  let resp = response.get()
  if resp.comments.isSome:
    echo "Found ", resp.comments.get().len, " comments"
```

### Utilizzo delle API di Moderazione (ModerationAPI)

Gli endpoint di moderazione alimentano la dashboard del moderatore e sono autenticati con un token SSO per il moderatore attivo:

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_moderation

let client = newHttpClient()

# Elenca i commenti nella dashboard di moderazione.
# Questa operazione non ha parametri richiesti, quindi tutto è opzionale.
let (response, httpResponse) = getApiComments(
  httpClient = client,
  options = GetApiCommentsOptions(
    count: 30,
    tenantId: "your-tenant-id",
    sso: "your-sso-token"
  )
)

if response.isSome:
  let resp = response.get()
  echo "Found ", resp.comments.len, " comments"
```

### Problemi comuni

1. **Errore di autenticazione 401**: Assicurati di impostare l'intestazione `x-api-key` sul tuo HttpClient prima di effettuare richieste DefaultAPI: `client.headers["x-api-key"] = "your-api-key"`
2. **Classe API errata**: Usa `api_default` per richieste autenticate lato server, `api_public` per richieste lato client/pubbliche, e `api_moderation` per richieste della dashboard del moderatore.