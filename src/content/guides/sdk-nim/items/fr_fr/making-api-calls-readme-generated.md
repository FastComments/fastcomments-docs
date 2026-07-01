---
Toutes les méthodes API de ce SDK renvoient des tuples de `(Option[ResponseType], Response)`. Le premier élément contient la réponse analysée en cas de succès, et le deuxième élément est la réponse HTTP brute.

Les paramètres obligatoires et le corps de la requête sont passés positionnellement. Les paramètres optionnels restants sont rassemblés dans un seul objet `Api<Operation>Options`, qui est le dernier argument. Les opérations sans paramètres optionnels ne prennent aucun objet d'options.

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
---