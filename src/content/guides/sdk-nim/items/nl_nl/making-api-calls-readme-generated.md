Alle API-methoden in deze SDK retourneren tuples van `(Option[ResponseType], Response)`. Het eerste element bevat de geparseerde respons indien geslaagd, en het tweede element is de ruwe HTTP-respons.

Vereiste parameters en de request body worden positioneel doorgegeven. De resterende optionele parameters worden verzameld in één `Api<Operation>Options`-object, dat het laatste argument is. Operaties zonder optionele parameters nemen geen opties-object.

### Voorbeeld: Opvragen van reacties

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