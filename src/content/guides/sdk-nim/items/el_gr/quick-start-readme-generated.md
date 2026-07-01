### Χρήση Πιστοποιημένων API (DefaultAPI)

**Σημαντικό:** Τα πιστοποιημένα σημεία λήψης απαιτούν το κλειδί API σας να οριστεί ως την κεφαλίδα `x-api-key`.

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

### Χρήση Δημοσίων API (PublicAPI)

Τα δημόσια σημεία λήψης δεν απαιτούν έλεγχο ταυτότητας:

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

### Χρήση API Εποπτείας (ModerationAPI)

Τα σημεία λήψης εποπτείας τροφοδοτούν τον πίνακα ελεγκτή και πιστοποιούνται με ένα token SSO για τον ενεργό ελεγκτή:

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

### Κοινά Προβλήματα

1. **Σφάλμα ελέγχου ταυτότητας 401**: Βεβαιωθείτε ότι έχετε ορίσει την κεφαλίδα `x-api-key` στο HttpClient σας πριν κάνετε αιτήματα DefaultAPI: `client.headers["x-api-key"] = "your-api-key"`
2. **Λάθος κλάση API**: Χρησιμοποιήστε `api_default` για αιτήματα πιστοποιημένα από τον διακομιστή, `api_public` για αιτήματα από τον πελάτη/δημόσια, και `api_moderation` για αιτήματα πίνακα ελεγκτή.