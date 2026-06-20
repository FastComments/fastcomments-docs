### Uporaba avtenticiranih API-jev (DefaultAPI)

**Pomembno:** Avtenticirane končne točke zahtevajo, da je vaš API ključ nastavljen v glavi `x-api-key`.

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_default
import fastcomments/models/model_comment_data

let client = newHttpClient()
client.headers["x-api-key"] = "your-api-key"

# Izvedi avtenticirane API klice
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

### Uporaba javnih API-jev (PublicAPI)

Javne končne točke ne zahtevajo avtentikacije:

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_public

let client = newHttpClient()

# Izvedi javne API klice
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

### Uporaba moderacijskih API-jev (ModerationAPI)

Moderacijske končne točke poganjajo nadzorno ploščo moderatorja in so avtenticirane z SSO žetonom aktivnega moderatorja:

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_moderation

let client = newHttpClient()

# Naštej komentarje v moderacijskem vmesniku
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

### Pogoste težave

1. **401 napaka avtentikacije**: Prepričajte se, da nastavite glavo `x-api-key` na svojem HttpClient pred izvajanjem DefaultAPI zahtev: `client.headers["x-api-key"] = "your-api-key"`
2. **Napačen razred API**: Uporabite `api_default` za strežniške avtenticirane zahteve, `api_public` za odjemalčeve/javne zahteve in `api_moderation` za zahteve nadzorne plošče moderatorja.