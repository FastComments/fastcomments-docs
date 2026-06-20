### Χρήση Αυθεντικοποιημένων API (DefaultAPI)

**Σημαντικό:** Τα αυθεντικοποιημένα endpoints απαιτούν το API key σας να οριστεί στην κεφαλίδα `x-api-key`.

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

### Χρήση Δημόσιων API (PublicAPI)

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

### Χρήση API Εποπτείας (ModerationAPI)

Τα endpoints εποπτείας τροφοδοτούν τον πίνακα ελέγχου των συντονιστών και αυθεντικοποιούνται με ένα SSO token για τον συντονιστή που ενεργεί:

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_moderation

let client = newHttpClient()

# Λίστα σχολίων στον πίνακα ελέγχου εποπτείας
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

### Συνηθισμένα Προβλήματα

1. **401 authentication error**: Βεβαιωθείτε ότι έχετε ορίσει την κεφαλίδα `x-api-key` στο HttpClient σας πριν κάνετε κλήσεις στην DefaultAPI: `client.headers["x-api-key"] = "your-api-key"`
2. **Wrong API class**: Χρησιμοποιήστε `api_default` για server-side αυθεντικοποιημένες αιτήσεις, `api_public` για client-side/δημόσιες αιτήσεις, και `api_moderation` για αιτήσεις του πίνακα ελέγχου των συντονιστών.