### Using Authenticated APIs (DefaultAPI)

**Важно:** Аутентифицированные конечные точки требуют, чтобы ваш API‑ключ был установлен в заголовке `x-api-key`.

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

### Using Public APIs (PublicAPI)

Public endpoints don't require authentication:

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

### Using Moderation APIs (ModerationAPI)

Moderation endpoints power the moderator dashboard and are authenticated with an SSO token for the acting moderator:

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

### Common Issues

1. **401 authentication error**: Make sure you set the `x-api-key` header on your HttpClient before making DefaultAPI requests: `client.headers["x-api-key"] = "your-api-key"`
2. **Wrong API class**: Use `api_default` for server-side authenticated requests, `api_public` for client-side/public requests, and `api_moderation` for moderator dashboard requests.