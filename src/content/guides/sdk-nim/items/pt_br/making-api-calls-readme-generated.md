All API methods in this SDK return tuples of `(Option[ResponseType], Response)`. O primeiro elemento contém a resposta analisada se for bem‑sucedida, e o segundo elemento é a resposta HTTP bruta.

Required parameters and the request body are passed positionally. Os parâmetros opcionais restantes são reunidos em um único objeto `Api<Operation>Options`, que é o último argumento. Operations with no optional parameters take no options object.

### Exemplo: Obtendo Comentários

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