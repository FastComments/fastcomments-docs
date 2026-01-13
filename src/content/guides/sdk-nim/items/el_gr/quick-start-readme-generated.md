### Χρήση Αυθεντικοποιημένων API (DefaultAPI)

**Σημαντικό:** Τα endpoints που απαιτούν αυθεντικοποίηση χρειάζονται το API key σας να οριστεί ως κεφαλίδα `x-api-key`.

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_default
import fastcomments/models/model_comment_data

let client = newHttpClient()
client.headers["x-api-key"] = "your-api-key"

# Εκτέλεση αυθεντικοποιημένων κλήσεων API
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

# Εκτέλεση δημόσιων κλήσεων API
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

### Συνηθισμένα προβλήματα

1. **401 authentication error**: Βεβαιωθείτε ότι έχετε ορίσει την κεφαλίδα `x-api-key` στο HttpClient σας πριν κάνετε αιτήματα DefaultAPI: `client.headers["x-api-key"] = "your-api-key"`
2. **Wrong API class**: Χρησιμοποιήστε `api_default` για αιτήματα με αυθεντικοποίηση από πλευράς διακομιστή, `api_public` για αιτήματα από πλευράς πελάτη/δημόσια.