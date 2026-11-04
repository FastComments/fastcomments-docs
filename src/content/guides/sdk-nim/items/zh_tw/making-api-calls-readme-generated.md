此 SDK 中的所有 API 方法皆回傳 `(Option[ResponseType], Response)` 的元組。第一個元素包含成功時的解析後回應，第二個元素則是原始的 HTTP 回應。

必要參數與請求主體以位置參數方式傳遞。其餘的選用參數會被收集到單一 `Api<Operation>Options` 物件中，作為最後一個參數。沒有選用參數的操作則不接受選項物件。

### 範例：取得留言

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