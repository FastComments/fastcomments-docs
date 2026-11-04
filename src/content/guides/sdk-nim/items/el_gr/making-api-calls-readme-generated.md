Όλες οι μέθοδοι API σε αυτό το SDK επιστρέφουν πλειάδες `(Option[ResponseType], Response)`. Το πρώτο στοιχείο περιέχει την αναλυμένη απάντηση σε περίπτωση επιτυχίας, και το δεύτερο στοιχείο είναι η ακατέργαστη HTTP απάντηση.

Οι υποχρεωτικές παράμετροι και το σώμα του αιτήματος περνούν θέτοντας θέση. Οι υπόλοιπες προαιρετικές παράμετροι συγκεντρώνονται σε ένα ενιαίο αντικείμενο `Api<Operation>Options`, το οποίο είναι το τελευταίο όρισμα. Οι λειτουργίες χωρίς προαιρετικές παραμέτρους δεν δέχονται αντικείμενο επιλογών.

### Παράδειγμα: Ανάκτηση Σχολίων

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