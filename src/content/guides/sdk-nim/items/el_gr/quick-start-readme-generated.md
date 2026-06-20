### Χρήση Αυθεντικοποιημένων APIs (DefaultAPI)

**Σημαντικό:** Τα αυθεντικοποιημένα endpoints απαιτούν να ορίσετε το API key σας ως την κεφαλίδα `x-api-key`.

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_default
import fastcomments/models/model_comment_data

let client = newHttpClient()
client.headers["x-api-key"] = "your-api-key"

# Κάντε αυθεντικοποιημένες κλήσεις API
let (response, httpResponse) = getComments(
  httpClient = client,
  tenantId = "your-tenant-id",
  page = 0,
  limit = 0,
  skip = 0,
  asTree = false,
  skipChildren = 0,
  limitChildren = 0,
  maxTreeDepth = 0,
  urlId = "your-url-id",
  userId = "",
  anonUserId = "",
  contextUserId = "",
  hashTag = "",
  parentId = "",
  direction = SortDirections.DESC
)

if response.isSome:
  let resp = response.get()
  if resp.comments.isSome:
    echo "Found ", resp.comments.get().len, " comments"
```

### Χρήση Δημόσιων APIs (PublicAPI)

Τα δημόσια endpoints δεν απαιτούν αυθεντικοποίηση:

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_public

let client = newHttpClient()

# Κάντε δημόσιες κλήσεις API
let (response, httpResponse) = getCommentsPublic(
  httpClient = client,
  tenantId = "your-tenant-id",
  urlId = "your-url-id",
  page = 0,
  direction = SortDirections.DESC,
  sso = "",
  skip = 0,
  skipChildren = 0,
  limit = 0,
  limitChildren = 0,
  countChildren = false,
  fetchPageForCommentId = "",
  includeConfig = false,
  countAll = false,
  includei10n = false,
  locale = "",
  modules = "",
  isCrawler = false,
  includeNotificationCount = false,
  asTree = false,
  maxTreeDepth = 0,
  useFullTranslationIds = false,
  parentId = "",
  searchText = "",
  hashTags = @[],
  userId = "",
  customConfigStr = "",
  afterCommentId = "",
  beforeCommentId = ""
)

if response.isSome:
  let resp = response.get()
  if resp.comments.isSome:
    echo "Found ", resp.comments.get().len, " comments"
```

### Χρήση APIs Εποπτείας (ModerationAPI)

Τα endpoints εποπτείας τροφοδοτούν τον πίνακα ελέγχου του συντονιστή και αυθεντικοποιούνται με ένα SSO token για τον ενεργούντα συντονιστή:

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_moderation

let client = newHttpClient()

# Λίστα σχολίων στον πίνακα εποπτείας
let (response, httpResponse) = getApiComments(
  httpClient = client,
  page = 0,
  count = 30,
  textSearch = "",
  byIPFromComment = "",
  filters = "",
  searchFilters = "",
  sorts = "",
  demo = false,
  sso = "your-sso-token"
)

if response.isSome:
  let resp = response.get()
  echo "Found ", resp.comments.len, " comments"
```

### Συνήθη Προβλήματα

1. **Σφάλμα 401 αυθεντικοποίησης**: Βεβαιωθείτε ότι έχετε ορίσει την κεφαλίδα `x-api-key` στο HttpClient σας πριν κάνετε αιτήσεις DefaultAPI: `client.headers["x-api-key"] = "your-api-key"`
2. **Λανθασμένη κλάση API**: Χρησιμοποιήστε `api_default` για αυθεντικοποιημένες αιτήσεις στην πλευρά του server, `api_public` για client-side/δημόσιες αιτήσεις, και `api_moderation` για αιτήσεις στον πίνακα ελέγχου του διαχειριστή.