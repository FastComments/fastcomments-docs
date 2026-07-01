---
Todos los métodos API en este SDK devuelven tuplas de `(Option[ResponseType], Response)`. El primer elemento contiene la respuesta analizada si tiene éxito, y el segundo elemento es la respuesta HTTP cruda.

Los parámetros obligatorios y el cuerpo de la solicitud se pasan posicionalmente. Los parámetros opcionales restantes se recopilan en un único objeto `Api<Operation>Options`, que es el último argumento. Las operaciones sin parámetros opcionales no usan objeto de opciones.

### Ejemplo: Obtención de Comentarios

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