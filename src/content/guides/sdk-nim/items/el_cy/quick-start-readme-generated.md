### Χρήση Αυθεντικοποιημένων API (DefaultAPI)

**Σημαντικό:** Οι αυθεντικοποιημένοι endpoints απαιτούν το API key σας να οριστεί ως κεφαλίδα `x-api-key`.

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

Οι δημόσιοι endpoints δεν απαιτούν αυθεντικοποίηση:

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

### Συνηθισμένα Προβλήματα

1. **401 authentication error**: Βεβαιωθείτε ότι έχετε ρυθμίσει την κεφαλίδα `x-api-key` στον HttpClient σας πριν κάνετε αιτήσεις DefaultAPI: `client.headers["x-api-key"] = "your-api-key"`
2. **Wrong API class**: Χρησιμοποιήστε `api_default` για αιτήσεις με αυθεντικοποίηση στην πλευρά του διακομιστή και `api_public` για αιτήσεις στην πλευρά του πελάτη/δημόσιες.