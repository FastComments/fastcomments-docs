此 SDK 中的所有 API 方法返回 `(Option[ResponseType], Response)` 元组。第一个元素在成功时包含解析后的响应，第二个元素是原始的 HTTP 响应。

必需的参数和请求体按位置传递。其余可选参数收集到一个 `Api<Operation>Options` 对象中，作为最后一个参数。没有可选参数的操作无需提供 options 对象。

### 示例：获取评论

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
---