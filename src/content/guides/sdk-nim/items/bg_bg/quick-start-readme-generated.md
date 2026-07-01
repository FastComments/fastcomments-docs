### Using Authenticated APIs (DefaultAPI)

**Важно:** Удостоверените крайни точки изискват вашият API ключ да бъде зададен като заглавка `x-api-key`.

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

Публичните крайни точки не изискват удостоверяване:

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

Крайните точки за модериране захранват таблото на модератора и са удостоверени със SSO токен за действащия модератор:

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

1. **401 грешка при удостоверяване**: Уверете се, че сте задали заглавката `x-api-key` на вашия HttpClient преди да правите заявки към DefaultAPI: `client.headers["x-api-key"] = "your-api-key"`
2. **Грешен API клас**: Използвайте `api_default` за сървърни удостоверени заявки, `api_public` за клиентски/публични заявки и `api_moderation` за заявки към таблото на модератора.