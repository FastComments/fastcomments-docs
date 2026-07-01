Sve API metode u ovom SDK‑u vraćaju tuple tipa `(Option[ResponseType], Response)`. Prvi element sadrži parsirani odgovor ako je uspješan, a drugi element je sirovi HTTP odgovor.

Obavezni parametri i tijelo zahtjeva se prosljeđuju pozicijski. Preostali opcionalni parametri se sakupljaju u jedan objekat `Api<Operation>Options`, koji je posljednji argument. Operacije bez opcionalnih parametara ne koriste objekat opcija.

### Primjer: Dohvaćanje komentara

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