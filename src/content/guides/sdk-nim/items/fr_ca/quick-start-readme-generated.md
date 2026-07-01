### Utilisation des APIs authentifiées (DefaultAPI)

**Important :** Les points de terminaison authentifiés exigent que votre clé API soit définie dans l’en‑tête `x-api-key`.

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_default
import fastcomments/models/model_comment_data

let client = newHttpClient()
client.headers["x-api-key"] = "your-api-key"

# Effectuer des appels API authentifiés.
# Les paramètres requis (et le corps de la requête) sont positionnels ; les
# paramètres sont passés via l'objet d'options de l'opération.
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

### Utilisation des APIs publiques (PublicAPI)

Les points de terminaison publics ne nécessitent pas d'authentification :

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

### Utilisation des APIs de modération (ModerationAPI)

Les points de terminaison de modération alimentent le tableau de bord des modérateurs et sont authentifiés avec un jeton SSO pour le modérateur agissant :

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

1. **Erreur d'authentification 401** : Assurez‑vous de définir l’en‑tête `x-api-key` sur votre HttpClient avant d’effectuer des requêtes DefaultAPI : `client.headers["x-api-key"] = "your-api-key"`
2. **Classe d'API incorrecte** : Utilisez `api_default` pour les requêtes authentifiées côté serveur, `api_public` pour les requêtes côté client/public, et `api_moderation` pour les requêtes du tableau de bord des modérateurs.