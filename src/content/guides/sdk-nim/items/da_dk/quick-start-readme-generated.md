### Brug af autentificerede API'er (DefaultAPI)

**Vigtigt:** Autentificerede slutpunkter kræver, at din API-nøgle er angivet som `x-api-key` headeren.

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_default
import fastcomments/models/model_comment_data

let client = newHttpClient()
client.headers["x-api-key"] = "your-api-key"

# Foretag autentificerede API‑kald.
# Påkrævede parametre (og anmodnings‑kroppen) er positions‑baserede; valgfrie
# parametre sendes via operationens options‑objekt.
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

### Brug af offentlige API'er (PublicAPI)

Offentlige slutpunkter kræver ikke autentificering:

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_public

let client = newHttpClient()

# Foretag offentlige API‑kald.
# tenantId og urlId er påkrævet (positions‑baseret); alt andet er valgfrit.
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

### Brug af moderations‑API'er (ModerationAPI)

Moderations‑endepunkter driver moderator‑dashboardet og er autentificerede med en SSO‑token for den agerende moderator:

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_moderation

let client = newHttpClient()

# List kommentarer i moderations‑dashboardet.
# Denne operation har ingen påkrævede parametre, så alt er valgfrit.
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

### Almindelige problemer

1. **401 autentificeringsfejl**: Sørg for at du angiver `x-api-key` headeren på din HttpClient, før du foretager DefaultAPI‑anmodninger: `client.headers["x-api-key"] = "your-api-key"`
2. **Forkert API‑klasse**: Brug `api_default` til server‑side autentificerede anmodninger, `api_public` til klient‑side/offentlige anmodninger, og `api_moderation` til moderator‑dashboard‑anmodninger.