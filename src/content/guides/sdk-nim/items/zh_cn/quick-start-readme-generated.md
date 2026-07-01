### 使用已认证的 APIs (DefaultAPI)

**重要：** 已认证的端点需要将您的 API 密钥设置为 `x-api-key` 请求头。

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_default
import fastcomments/models/model_comment_data

let client = newHttpClient()
client.headers["x-api-key"] = "your-api-key"

# 发起已认证的 API 调用。
# 必需参数（以及请求体）是位置参数；可选
# 参数通过操作的 options 对象传递。
let (response, httpResponse) = getComments(
  httpClient = client,
  tenantId = "your-tenant-id",
  options = GetCommentsOptions(
    urlId: "your-url-id",
    direction: SortDirections.DESC
  )
)

if response.isSome:
  let resp = response.get()
  if resp.comments.isSome:
    echo "Found ", resp.comments.get().len, " comments"
```

### 使用公共 APIs (PublicAPI)

公共端点不需要身份验证：

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_public

let client = newHttpClient()

# 发起公共 API 调用。
# tenantId 和 urlId 为必需（位置参数）；其余均为可选。
let (response, httpResponse) = getCommentsPublic(
  httpClient = client,
  tenantId = "your-tenant-id",
  urlId = "your-url-id",
  options = GetCommentsPublicOptions(
    direction: SortDirections.DESC
  )
)

if response.isSome:
  let resp = response.get()
  if resp.comments.isSome:
    echo "Found ", resp.comments.get().len, " comments"
```

### 使用审查 APIs (ModerationAPI)

审查端点为审查员仪表板提供功能，并使用执行审查员的 SSO 令牌进行身份验证：

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_moderation

let client = newHttpClient()

# 列出审查仪表板中的评论。
# 此操作没有必需参数，所有参数均为可选。
let (response, httpResponse) = getApiComments(
  httpClient = client,
  options = GetApiCommentsOptions(
    count: 30,
    tenantId: "your-tenant-id",
    sso: "your-sso-token"
  )
)

if response.isSome:
  let resp = response.get()
  echo "Found ", resp.comments.len, " comments"
```

### 常见问题

1. **401 认证错误**：确保在进行 DefaultAPI 请求之前在 HttpClient 上设置 `x-api-key` 请求头：`client.headers["x-api-key"] = "your-api-key"`
2. **错误的 API 类**：对于服务端已认证请求使用 `api_default`，对于客户端/公共请求使用 `api_public`，对于审查仪表板请求使用 `api_moderation`。