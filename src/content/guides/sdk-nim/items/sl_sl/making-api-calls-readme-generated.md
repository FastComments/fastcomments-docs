All API metode v tem SDK-ju vrnejo nizek `(Option[ResponseType], Response)`. Prvi element vsebuje razčlenjen odgovor, če je uspešen, drugi element pa je surov HTTP odziv.

Obvezni parametri in telo zahteve se posredujejo pozicijsko. Preostali neobvezni parametri se zberejo v en sam objekt `Api<Operation>Options`, ki je zadnji argument. Operacije brez neobveznih parametrov ne uporabljajo objekta možnosti.

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