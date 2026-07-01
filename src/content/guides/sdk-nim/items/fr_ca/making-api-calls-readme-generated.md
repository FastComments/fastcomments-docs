Toutes les méthodes d’API dans ce SDK renvoient des tuples de `(Option[ResponseType], Response)`. Le premier élément contient la réponse analysée si la requête réussit, et le deuxième élément est la réponse HTTP brute.

Les paramètres requis et le corps de la requête sont passés positionnellement. Les paramètres optionnels restants sont regroupés dans un seul objet `Api<Operation>Options`, qui constitue le dernier argument. Les opérations qui n’ont pas de paramètres optionnels n’utilisent aucun objet d’options.

### Exemple : Récupération des commentaires

```nim
import httpclient
import options
import fastcomments
import fastcomments/apis/api_default

let client = newHttpClient()
client.headers["x-api-key"] = "your-api-key"

let (response, httpResponse) = getComments(
  httpClient = client,
  tenantId = "your-tenant-id",
  options = GetCommentsOptions(
    urlId: "your-url-id",
    direction: SortDirections.DESC
  )
)

if httpResponse.code == Http200:
  if response.isSome:
    let resp = response.get()
    if resp.comments.isSome:
      echo "Found ", resp.comments.get().len, " comments"
```