### Utilisation des API authentifiées (DefaultAPI)

**Important :** Les points de terminaison authentifiés exige que votre clé API soit définie en tant qu'en-tête `x-api-key`.

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_default
import fastcomments/models/model_comment_data

let client = newHttpClient()
client.headers["x-api-key"] = "your-api-key"

# Effectuer des appels d'API authentifiés
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

Les points de terminaison publics n'exigent pas d'authentification :

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_public

let client = newHttpClient()

# Effectuer des appels d'API publics
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

### Utilisation des API de modération (ModerationAPI)

Les points de terminaison de modération alimentent le tableau de bord du modérateur et sont authentifiés avec un jeton SSO pour le modérateur en action :

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_moderation

let client = newHttpClient()

# Lister les commentaires dans le tableau de bord de modération
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

### Problèmes courants

1. **Erreur d'authentification 401** : Assurez-vous de définir l'en-tête `x-api-key` sur votre HttpClient avant d'effectuer des requêtes DefaultAPI : `client.headers["x-api-key"] = "your-api-key"`
2. **Mauvaise classe d'API** : Utilisez `api_default` pour les requêtes authentifiées côté serveur, `api_public` pour les requêtes côté client/publiques, et `api_moderation` pour les requêtes du tableau de bord du modérateur.