このSDKのすべてのAPIメソッドは `(Option[ResponseType], Response)` のタプルを返します。最初の要素には成功した場合に解析されたレスポンスが含まれ、二番目の要素は生のHTTPレスポンスです。

必須パラメータとリクエストボディは位置引数として渡されます。残りのオプションパラメータは単一の `Api<Operation>Options` オブジェクトにまとめられ、これが最後の引数となります。オプションパラメータがない操作はオプションオブジェクトを取らないことになります。

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