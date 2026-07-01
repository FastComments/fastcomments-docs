### Utilisation des API authentifiées (DefaultAPI)

**Important :** Authenticated endpoints require your API key to be set as the `x-api-key` header.

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_default
import fastcomments/models/model_comment_data

let client = newHttpClient()
client.headers["x-api-key"] = "your-api-key"

# Effectuer des appels API authentifiés.
# Les paramètres requis (et le corps de la requête) sont positionnels ; les paramètres optionnels sont passés via l'objet options de l'opération.
let (response, httpResponse) = getComments(
  httpClient = client,
  tenantId = "your-tenant-id",
  options = GetCommentsOptions(
    urlId: "your-url-id",
    direction: SortDirections.DESC
  )
)

if response.isSome:
  let resp = response.get()
  if resp.comments.isSome:
    echo "Found ", resp.comments.get().len, " comments"
```

### Utilisation des API publiques (PublicAPI)

Public endpoints don't require authentication:

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_public

let client = newHttpClient()

# Effectuer des appels API publics.
# tenantId et urlId sont requis (positionnels) ; tout le reste est optionnel.
let (response, httpResponse) = getCommentsPublic(
  httpClient = client,
  tenantId = "your-tenant-id",
  urlId = "your-url-id",
  options = GetCommentsPublicOptions(
    direction: SortDirections.DESC
  )
)

if response.isSome:
  let resp = response.get()
  if resp.comments.isSome:
    echo "Found ", resp.comments.get().len, " comments"
```

### Utilisation des API de modération (ModerationAPI)

Moderation endpoints power the moderator dashboard and are authenticated with an SSO token for the acting moderator:

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_moderation

let client = newHttpClient()

# Lister les commentaires dans le tableau de bord de modération.
# Cette opération n'a aucun paramètre requis, donc tout est optionnel.
let (response, httpResponse) = getApiComments(
  httpClient = client,
  options = GetApiCommentsOptions(
    count: 30,
    tenantId: "your-tenant-id",
    sso: "your-sso-token"
  )
)

if response.isSome:
  let resp = response.get()
  echo "Found ", resp.comments.len, " comments"
```

### Problèmes courants

1. **401 error d'authentification** : Assurez‑vous de définir l'en-tête `x-api-key` sur votre HttpClient avant d'effectuer des requêtes DefaultAPI : `client.headers["x-api-key"] = "your-api-key"`
2. **Classe API incorrecte** : Utilisez `api_default` pour les requêtes authentifiées côté serveur, `api_public` pour les requêtes côté client/public, et `api_moderation` pour les requêtes du tableau de bord des modérateurs.