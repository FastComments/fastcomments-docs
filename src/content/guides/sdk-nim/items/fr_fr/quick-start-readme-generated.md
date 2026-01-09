### Utilisation des API authentifiées (DefaultAPI)

**Important :** Les endpoints authentifiés nécessitent que votre clé API soit définie en tant qu'en-tête `x-api-key`.

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_default
import fastcomments/models/model_comment_data

let client = newHttpClient()
client.headers["x-api-key"] = "your-api-key"

# Effectuer des appels API authentifiés
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

### Utilisation des API publiques (PublicAPI)

Les endpoints publics ne requièrent pas d'authentification :

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_public

let client = newHttpClient()

# Effectuer des appels API publics
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

### Problèmes courants

1. **401 authentication error** : Assurez-vous de définir l'en-tête `x-api-key` sur votre HttpClient avant d'effectuer des requêtes DefaultAPI : `client.headers["x-api-key"] = "your-api-key"`
2. **Wrong API class** : Utilisez `api_default` pour les requêtes authentifiées côté serveur, `api_public` pour les requêtes côté client/publiques.