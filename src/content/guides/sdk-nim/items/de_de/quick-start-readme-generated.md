### Verwendung authentifizierter APIs (DefaultAPI)

**Wichtig:** Authentifizierte Endpunkte erfordern, dass Ihr API‑Schlüssel als Header `x-api-key` gesetzt wird.

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_default
import fastcomments/models/model_comment_data

let client = newHttpClient()
client.headers["x-api-key"] = "your-api-key"

# Authentifizierte API‑Aufrufe durchführen.
# Erforderliche Parameter (und der Anfrage‑Body) sind positional; optionale
# Parameter werden über das Options‑Objekt der Operation übergeben.
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

### Verwendung öffentlicher APIs (PublicAPI)

Öffentliche Endpunkte erfordern keine Authentifizierung:

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_public

let client = newHttpClient()

# Öffentliche API‑Aufrufe durchführen.
# tenantId und urlId sind erforderlich (positional); alles andere ist optional.
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

### Verwendung von Moderations‑APIs (ModerationAPI)

Moderations‑Endpunkte versorgen das Moderator‑Dashboard und sind mit einem SSO‑Token für den handelnden Moderator authentifiziert:

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_moderation

let client = newHttpClient()

# Kommentare im Moderations‑Dashboard auflisten.
# Diese Operation hat keine erforderlichen Parameter, daher ist alles optional.
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

### Häufige Probleme

1. **401 Authentifizierungsfehler**: Stellen Sie sicher, dass Sie den Header `x-api-key` in Ihrem HttpClient setzen, bevor Sie DefaultAPI‑Anfragen stellen: `client.headers["x-api-key"] = "your-api-key"`
2. **Falsche API‑Klasse**: Verwenden Sie `api_default` für serverseitige authentifizierte Anfragen, `api_public` für clientseitige/öffentliche Anfragen und `api_moderation` für Anfragen an das Moderator‑Dashboard.