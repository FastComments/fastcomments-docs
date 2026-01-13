### Verwendung authentifizierter APIs (DefaultAPI)

**Wichtig:** Authentifizierte Endpunkte erfordern, dass Ihr API-Schlüssel als `x-api-key` Header gesetzt ist.

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

### Häufige Probleme

1. **401 Authentifizierungsfehler**: Stellen Sie sicher, dass Sie den `x-api-key`-Header in Ihrem HttpClient setzen, bevor Sie DefaultAPI-Anfragen stellen: `client.headers["x-api-key"] = "your-api-key"`
2. **Falsche API-Klasse**: Verwenden Sie `api_default` für serverseitige authentifizierte Anfragen, `api_public` für clientseitige/öffentliche Anfragen.
---