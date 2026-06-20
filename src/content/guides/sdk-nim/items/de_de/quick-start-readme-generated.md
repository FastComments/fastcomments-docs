### Verwendung authentifizierter APIs (DefaultAPI)

**Wichtig:** Authentifizierte Endpunkte erfordern, dass Ihr API-Schlüssel als Header `x-api-key` gesetzt ist.

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_default
import fastcomments/models/model_comment_data

let client = newHttpClient()
client.headers["x-api-key"] = "your-api-key"

# Authentifizierte API-Aufrufe durchführen
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

### Verwendung öffentlicher APIs (PublicAPI)

Öffentliche Endpunkte erfordern keine Authentifizierung:

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_public

let client = newHttpClient()

# Öffentliche API-Aufrufe durchführen
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

### Verwendung der Moderations-APIs (ModerationAPI)

Moderations-Endpunkte versorgen das Moderatoren-Dashboard und werden mit einem SSO-Token für den handelnden Moderator authentifiziert:

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_moderation

let client = newHttpClient()

# Kommentare im Moderations-Dashboard auflisten
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

### Häufige Probleme

1. **401-Authentifizierungsfehler**: Stellen Sie sicher, dass Sie den Header `x-api-key` auf Ihrem HttpClient setzen, bevor Sie DefaultAPI-Anfragen stellen: `client.headers["x-api-key"] = "your-api-key"`
2. **Falsche API-Klasse**: Verwenden Sie `api_default` für serverseitige authentifizierte Anfragen, `api_public` für clientseitige/öffentliche Anfragen, und `api_moderation` für Anfragen des Moderations-Dashboards.