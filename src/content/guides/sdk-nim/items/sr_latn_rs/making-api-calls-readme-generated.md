All API metode u ovom SDK‑u vraćaju torke `(Option[ResponseType], Response)`. Prvi element sadrži parsirani odgovor ako je uspešan, a drugi element je sirovi HTTP odgovor.

Obavezni parametri i telo zahteva se prosleđuju po položaju. Preostali opcioni parametri se skupljaju u jedan objekat `Api<Operation>Options`, koji je poslednji argument. Operacije bez opcionih parametara ne uzimaju objekat opcija.

### Primer: Dohvatanje komentara

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