### Χρήση Πιστοποιημένων APIs (DefaultAPI)

**Σημαντικό:** Τα πιστοποιημένα σημεία λήψης απαιτούν το κλειδί API σας να ορίζεται ως η κεφαλίδα `x-api-key`.

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_default
import fastcomments/models/model_comment_data

let client = newHttpClient()
client.headers["x-api-key"] = "your-api-key"

# Κάντε κλήσεις πιστοποιημένων API.
# Τα απαιτούμενα παραμέτρους (και το σώμα της αίτησης) είναι διατεταγμένα· προαιρετικές
# παράμετροι περνούν μέσω του αντικειμένου επιλογών της λειτουργίας.
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

### Χρήση Δημόσιων APIs (PublicAPI)

Τα δημόσια σημεία λήψης δεν απαιτούν πιστοποίηση:

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_public

let client = newHttpClient()

# Κάντε κλήσεις δημόσιων API.
# tenantId και urlId είναι απαιτούμενα (διατεταγμένα)· όλα τα υπόλοιπα είναι προαιρετικά.
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

### Χρήση API Συντονισμού (ModerationAPI)

Τα σημεία λήψης συντονισμού τροφοδοτούν τον πίνακα ελέγχου συντονιστή και πιστοποιούνται με διακριτικό SSO για τον ενεργό συντονιστή:

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_moderation

let client = newHttpClient()

# Καταχωρίστε σχόλια στον πίνακα ελέγχου συντονισμού.
# Αυτή η λειτουργία δεν έχει απαιτούμενες παραμέτρους, επομένως όλα είναι προαιρετικά.
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

1. **401 σφάλμα πιστοποίησης**: Βεβαιωθείτε ότι έχετε ορίσει την κεφαλίδα `x-api-key` στο HttpClient σας πριν κάνετε αιτήσεις DefaultAPI: `client.headers["x-api-key"] = "your-api-key"`
2. **Λάθος κλάση API**: Χρησιμοποιήστε `api_default` για αιτήματα πιστοποιημένα από την πλευρά του διακομιστή, `api_public` για αιτήματα από την πλευρά του πελάτη/δημόσια, και `api_moderation` για αιτήματα πίνακα ελέγχου συντονιστή.