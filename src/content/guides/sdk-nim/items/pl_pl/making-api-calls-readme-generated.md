Wszystkie metody API w tym SDK zwracają krotki `(Option[ResponseType], Response)`. Pierwszy element zawiera sparsowaną odpowiedź w przypadku sukcesu, a drugi element to surowa odpowiedź HTTP.

Wymagane parametry i ciało żądania są przekazywane pozycyjnie. Pozostałe opcjonalne parametry są zbierane w pojedynczy obiekt `Api<Operation>Options`, który jest ostatnim argumentem. Operacje bez parametrów opcjonalnych nie przyjmują obiektu opcji.

### Przykład: Pobieranie komentarzy

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