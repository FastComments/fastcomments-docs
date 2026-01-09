Vse API metode v tem SDK vračajo pare `(Option[ResponseType], Response)`. Prvi element vsebuje razčlenjen odgovor, če je zahteva uspešna, drugi element pa je surovi HTTP odgovor.

### Primer: Pridobivanje komentarjev

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

if httpResponse.code == Http200:
  if response.isSome:
    let resp = response.get()
    if resp.comments.isSome:
      echo "Found ", resp.comments.get().len, " comments"
```