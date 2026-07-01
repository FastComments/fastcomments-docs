### Using Authenticated APIs (DefaultAPI)

**Important:** Authenticated endpoints require your API key to be set as the `x-api-key` header.

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_default
import fastcomments/models/model_comment_data

let client = newHttpClient()
client.headers["x-api-key"] = "your-api-key"

# Wykonaj uwierzytelnione wywołania API.
# Wymagane parametry (i ciało żądania) są pozycyjne; opcjonalne
# parametry są przekazywane za pomocą obiektu opcji operacji.
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

# Wykonaj publiczne wywołania API.
# tenantId i urlId są wymagane (pozycyjne); wszystko inne jest opcjonalne.
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

# Lista komentarzy w panelu moderacji.
# Ta operacja nie ma wymaganych parametrów, więc wszystko jest opcjonalne.
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