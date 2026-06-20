### Korzystanie z uwierzytelnionych API (DefaultAPI)

**Ważne:** Uwierzytelnione końcówki wymagają ustawienia Twojego klucza API w nagłówku `x-api-key`.

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_default
import fastcomments/models/model_comment_data

let client = newHttpClient()
client.headers["x-api-key"] = "your-api-key"

# Wykonaj uwierzytelnione wywołania API
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

### Korzystanie z publicznych API (PublicAPI)

Publiczne końcówki nie wymagają uwierzytelnienia:

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_public

let client = newHttpClient()

# Wykonaj publiczne wywołania API
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

### Korzystanie z API moderacji (ModerationAPI)

Końcówki moderacyjne zasilają panel moderatora i są uwierzytelniane tokenem SSO dla działającego moderatora:

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_moderation

let client = newHttpClient()

# Wyświetl komentarze w panelu moderacji
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

### Częste problemy

1. **401 authentication error**: Upewnij się, że ustawiłeś nagłówek `x-api-key` w swoim HttpClient przed wykonywaniem żądań DefaultAPI: `client.headers["x-api-key"] = "your-api-key"`
2. **Wrong API class**: Używaj `api_default` dla uwierzytelnionych żądań po stronie serwera, `api_public` dla żądań po stronie klienta/publicznych oraz `api_moderation` dla żądań panelu moderatora.
---