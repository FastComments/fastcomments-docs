### Korišćenje autentifikovanih API-ja (DefaultAPI)

**Važno:** Autentifikovani endpointi zahtevaju da vaš API ključ bude postavljen kao zaglavlje `x-api-key`.

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_default
import fastcomments/models/model_comment_data

let client = newHttpClient()
client.headers["x-api-key"] = "your-api-key"

# Izvršite autentifikovane API pozive
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

### Korišćenje javnih API-ja (PublicAPI)

Javni endpointi ne zahtevaju autentifikaciju:

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

### Korišćenje API-ja za moderaciju (ModerationAPI)

Moderacijski endpointi pokreću kontrolnu tablu moderatora i autentifikuju se pomoću SSO tokena za moderatora koji deluje:

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_moderation

let client = newHttpClient()

# Prikaži komentare na kontrolnoj tabli moderatora
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

### Uobičajeni problemi

1. **401 greška autentifikacije**: Uverite se da ste postavili zaglavlje `x-api-key` na vašem HttpClient pre nego što izvršite zahteve DefaultAPI-ja: `client.headers["x-api-key"] = "your-api-key"`
2. **Pogrešna API klasa**: Koristite `api_default` za autentifikovane zahteve na serverskoj strani, `api_public` za zahteve sa klijentske strane/javne zahteve, i `api_moderation` za zahteve kontrolne table moderatora.