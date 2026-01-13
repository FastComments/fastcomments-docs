### Kimlik Doğrulamalı API'leri Kullanma (DefaultAPI)

**Önemli:** Kimlik doğrulamalı uç noktalar için API anahtarınızın `x-api-key` başlığı olarak ayarlanması gerekir.

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_default
import fastcomments/models/model_comment_data

let client = newHttpClient()
client.headers["x-api-key"] = "your-api-key"

# Kimlik doğrulamalı API çağrıları yap
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

### Halka Açık API'leri Kullanma (PublicAPI)

Halka açık uç noktalar kimlik doğrulama gerektirmez:

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_public

let client = newHttpClient()

# Halka açık API çağrıları yap
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

### Yaygın Sorunlar

1. **401 authentication error**: DefaultAPI istekleri yapmadan önce HttpClient üzerinde `x-api-key` başlığını ayarladığınızdan emin olun: `client.headers["x-api-key"] = "your-api-key"`
2. **Wrong API class**: Sunucu tarafı kimlik doğrulamalı istekler için `api_default`, istemci tarafı/halka açık istekler için `api_public` kullanın.