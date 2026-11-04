このSDKのすべてのAPIメソッドは、`(Option[ResponseType], Response)` のタプルを返します。最初の要素には成功した場合のパース済みレスポンスが含まれ、2番目の要素は生のHTTPレスポンスです。

必須パラメーターとリクエストボディは位置引数として渡されます。残りのオプションパラメーターは単一の `Api<Operation>Options` オブジェクトにまとめられ、最後の引数として渡されます。オプションパラメーターを持たない操作は、optionsオブジェクトを受け取りません。

### 例: コメントの取得

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