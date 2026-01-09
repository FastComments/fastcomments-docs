### Korištenje autentificiranih API-ja (DefaultAPI)

**Važno:** Autentificirani endpointi zahtijevaju da vaš API ključ bude postavljen kao zaglavlje `x-api-key`.

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_default
import fastcomments/models/model_comment_data

let client = newHttpClient()
client.headers["x-api-key"] = "your-api-key"

# Izvršite autentificirane API pozive
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

### Korištenje javnih API-ja (PublicAPI)

Javni endpointi ne zahtijevaju autentifikaciju:

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_public

let client = newHttpClient()

# Izvršite javne API pozive
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

### Uobičajeni problemi

1. **401 authentication error**: Provjerite jeste li postavili zaglavlje `x-api-key` na vaš HttpClient prije slanja zahtjeva DefaultAPI-ja: `client.headers["x-api-key"] = "your-api-key"`
2. **Wrong API class**: Koristite `api_default` za autentificirane zahtjeve na strani servera, `api_public` za zahtjeve na strani klijenta/javne zahtjeve.