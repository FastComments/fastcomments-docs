All API methods in this SDK return tuples of `(Option[ResponseType], Response)`. The first element contains the parsed response if successful, and the second element is the raw HTTP response.

Required parameters and the request body are passed positionally. The remaining optional parameters are collected into a single `Api<Operation>Options` object, which is the last argument. Operations with no optional parameters take no options object.

### 範例：取得評論

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