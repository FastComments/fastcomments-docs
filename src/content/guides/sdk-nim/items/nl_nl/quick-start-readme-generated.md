### Gebruik van geauthenticeerde API's (DefaultAPI)

**Belangrijk:** Geauthenticeerde eindpunten vereisen dat uw API‑sleutel wordt ingesteld als de `x-api-key` header.

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_default
import fastcomments/models/model_comment_data

let client = newHttpClient()
client.headers["x-api-key"] = "your-api-key"

# Make authenticated API calls.
# Required parameters (and the request body) are positional; optional
# parameters are passed via the operation's options object.
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

### Gebruik van openbare API's (PublicAPI)

Openbare eindpunten vereisen geen authenticatie:

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_public

let client = newHttpClient()

# Make public API calls.
# tenantId and urlId are required (positional); everything else is optional.
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

### Gebruik van moderatie‑API's (ModerationAPI)

Moderatie‑eindpunten voeden het moderator‑dashboard en zijn geauthenticeerd met een SSO‑token voor de werkende moderator:

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_moderation

let client = newHttpClient()

# List comments in the moderation dashboard.
# This operation has no required parameters, so everything is optional.
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

### Veelvoorkomende problemen

1. **401 authenticatiefout**: Zorg ervoor dat u de `x-api-key` header op uw HttpClient instelt voordat u DefaultAPI‑verzoeken doet: `client.headers["x-api-key"] = "your-api-key"`
2. **Verkeerde API‑klasse**: Gebruik `api_default` voor server‑side geauthenticeerde verzoeken, `api_public` voor client‑side/openbare verzoeken, en `api_moderation` voor moderator‑dashboard verzoeken.