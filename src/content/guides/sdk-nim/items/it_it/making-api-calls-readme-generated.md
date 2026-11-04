Tutti i metodi API in questo SDK restituiscono tuple di `(Option[ResponseType], Response)`. Il primo elemento contiene la risposta analizzata in caso di successo, mentre il secondo elemento è la risposta HTTP grezza.

I parametri obbligatori e il corpo della richiesta vengono passati posizionalmente. I restanti parametri opzionali sono raccolti in un unico oggetto `Api<Operation>Options`, che è l'ultimo argomento. Le operazioni senza parametri opzionali non accettano alcun oggetto di opzioni.

### Esempio: Recupero dei Commenti

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