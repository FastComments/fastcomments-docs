All API methods in this SDK return tuples of `(Option[ResponseType], Response)`. Το πρώτο στοιχείο περιέχει την αναλυμένη απάντηση εάν είναι επιτυχής, και το δεύτερο στοιχείο είναι η ακατέργαστη απάντηση HTTP.

Οι απαιτούμενες παράμετροι και το σώμα του αιτήματος περνιούνται κατά θέση. Οι υπόλοιπες προαιρετικές παράμετροι συγκεντρώνονται σε ένα ενιαίο αντικείμενο `Api<Operation>Options`, το οποίο είναι το τελευταίο όρισμα. Οι λειτουργίες χωρίς προαιρετικές παραμέτρους δεν λαμβάνουν αντικείμενο επιλογών.

### Example: Fetching Comments

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