### 認証済み API の使用 (DefaultAPI)

**重要:** 認証済みエンドポイントでは、API キーを `x-api-key` ヘッダーに設定する必要があります。

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_default
import fastcomments/models/model_comment_data

let client = newHttpClient()
client.headers["x-api-key"] = "your-api-key"

# 認証済み API 呼び出しを行います。
# 必須パラメータ（およびリクエストボディ）は位置指定です；オプション
# パラメータは操作のオプションオブジェクトを介して渡されます。
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

### パブリック API の使用 (PublicAPI)

パブリックエンドポイントは認証を必要としません:

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_public

let client = newHttpClient()

# パブリック API 呼び出しを行います。
# tenantId と urlId は必須（位置指定）です；それ以外はすべてオプションです。
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

### モデレーション API の使用 (ModerationAPI)

モデレーションエンドポイントはモデレーター ダッシュボードに機能し、アクティブなモデレーターの SSO トークンで認証されます:

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_moderation

let client = newHttpClient()

# モデレーション ダッシュボードのコメントを一覧表示します。
# この操作には必須パラメータがないため、すべてオプションです。
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

### 共通の問題

1. **401 認証エラー**: DefaultAPI リクエストを行う前に HttpClient の `x-api-key` ヘッダーを設定してください: `client.headers["x-api-key"] = "your-api-key"`
2. **誤った API クラス**: サーバー側の認証リクエストには `api_default`、クライアント側/パブリックリクエストには `api_public`、モデレーター ダッシュボードのリクエストには `api_moderation` を使用してください。