### Geauthenticeerde API's gebruiken (DefaultAPI)

**Belangrijk:** Geauthenticeerde endpoints vereisen dat uw API-sleutel is ingesteld als de `x-api-key` header.

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_default
import fastcomments/models/model_comment_data

let client = newHttpClient()
client.headers["x-api-key"] = "your-api-key"

# Voer geauthenticeerde API-aanroepen uit
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

### Openbare API's gebruiken (PublicAPI)

Openbare endpoints vereisen geen authenticatie:

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_public

let client = newHttpClient()

# Voer openbare API-aanroepen uit
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

### Moderatie-API's gebruiken (ModerationAPI)

Moderatie-endpoints voorzien het moderatiedashboard en worden geauthenticeerd met een SSO-token voor de actieve moderator:

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_moderation

let client = newHttpClient()

# Lijst reacties in het moderatiedashboard
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

### Veelvoorkomende problemen

1. **401 authenticatiefout**: Zorg dat u de `x-api-key` header op uw HttpClient instelt voordat u DefaultAPI-aanvragen doet: `client.headers["x-api-key"] = "your-api-key"`
2. **Verkeerde API-klasse**: Gebruik `api_default` voor server-side geauthenticeerde verzoeken, `api_public` voor client-side/openbare verzoeken, en `api_moderation` voor verzoeken van het moderatiedashboard.