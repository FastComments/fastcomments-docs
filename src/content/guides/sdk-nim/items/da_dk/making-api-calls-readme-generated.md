Alle API‑metoder i dette SDK returnerer tupler af `(Option[ResponseType], Response)`. Det første element indeholder det parsede svar, hvis succesfuldt, og det andet element er den rå HTTP‑respons.

Påkrævede parametre og request‑body overføres positionsmæssigt. De resterende valgfrie parametre samles i et enkelt `Api<Operation>Options`‑objekt, som er det sidste argument. Operationer uden valgfrie parametre tager ikke et options‑objekt.

### Eksempel: Hentning af kommentarer

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