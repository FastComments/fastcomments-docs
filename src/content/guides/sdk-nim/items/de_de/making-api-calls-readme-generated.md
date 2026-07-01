All API‑Methoden in diesem SDK geben Tupel von `(Option[ResponseType], Response)` zurück. Das erste Element enthält die geparste Antwort, wenn sie erfolgreich ist, und das zweite Element ist die rohe HTTP‑Antwort.

Erforderliche Parameter und der Request‑Body werden positionsbasiert übergeben. Die übrigen optionalen Parameter werden in einem einzelnen `Api<Operation>Options`‑Objekt gesammelt, das das letzte Argument ist. Vorgänge ohne optionale Parameter benötigen kein Options‑Objekt.

### Beispiel: Abrufen von Kommentaren

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