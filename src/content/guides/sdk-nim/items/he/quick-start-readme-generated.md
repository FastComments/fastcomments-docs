---
### שימוש בממשקי API מאומתים (DefaultAPI)

**חשוב:** קצוות מאומתים דורשים שהמפתח API שלך יוגדר ככותרת `x-api-key`.

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

### שימוש בממשקי API ציבוריים (PublicAPI)

קצוות ציבוריים אינם דורשים אימות:

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

### שימוש בממשקי API למודרציה (ModerationAPI)

קצוות המודרציה מניעים את לוח הבקרה של המנחה ומאומתים בעזרת אסימון SSO למנחה הפועל:

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

### בעיות נפוצות

1. **שגיאת אימות 401**: ודא שהגדרת את כותרת `x-api-key` ב-HttpClient שלך לפני ביצוע בקשות DefaultAPI: `client.headers["x-api-key"] = "your-api-key"`
2. **מחלקת API שגויה**: השתמש ב-`api_default` לבקשות מאומתות בצד השרת, `api_public` לבקשות בצד הלקוח/ציבוריות, ו-`api_moderation` לבקשות של לוח הבקרה של המנחה.
---