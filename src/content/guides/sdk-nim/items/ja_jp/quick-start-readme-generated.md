### 認証されたAPIの使用 (DefaultAPI)

**重要:** 認証が必要なエンドポイントでは、APIキーを `x-api-key` ヘッダーとして設定する必要があります。

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_default
import fastcomments/models/model_comment_data

let client = newHttpClient()
client.headers["x-api-key"] = "your-api-key"

# 認証されたAPI呼び出しを行う
let (response, httpResponse) = getComments(
  httpClient = client,
  tenantId = "your-tenant-id",
  page = 0,
  limit = 0,
  skip = 0,
  asTree = false,
  skipChildren = 0,
  limitChildren = 0,
  maxTreeDepth = 0,
  urlId = "your-url-id",
  userId = "",
  anonUserId = "",
  contextUserId = "",
  hashTag = "",
  parentId = "",
  direction = SortDirections.DESC
)

if response.isSome:
  let resp = response.get()
  if resp.comments.isSome:
    echo "Found ", resp.comments.get().len, " comments"
```

### パブリックAPIの使用 (PublicAPI)

パブリック（公開）エンドポイントは認証を必要としません：

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_public

let client = newHttpClient()

# パブリックAPI呼び出しを行う
let (response, httpResponse) = getCommentsPublic(
  httpClient = client,
  tenantId = "your-tenant-id",
  urlId = "your-url-id",
  page = 0,
  direction = SortDirections.DESC,
  sso = "",
  skip = 0,
  skipChildren = 0,
  limit = 0,
  limitChildren = 0,
  countChildren = false,
  fetchPageForCommentId = "",
  includeConfig = false,
  countAll = false,
  includei10n = false,
  locale = "",
  modules = "",
  isCrawler = false,
  includeNotificationCount = false,
  asTree = false,
  maxTreeDepth = 0,
  useFullTranslationIds = false,
  parentId = "",
  searchText = "",
  hashTags = @[],
  userId = "",
  customConfigStr = "",
  afterCommentId = "",
  beforeCommentId = ""
)

if response.isSome:
  let resp = response.get()
  if resp.comments.isSome:
    echo "Found ", resp.comments.get().len, " comments"
```

### モデレーションAPIの使用 (ModerationAPI)

モデレーション用エンドポイントはモデレーターダッシュボードで使用され、操作を行うモデレータのSSOトークンで認証されます：

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_moderation

let client = newHttpClient()

# モデレーションダッシュボードのコメント一覧を取得する
let (response, httpResponse) = getApiComments(
  httpClient = client,
  page = 0,
  count = 30,
  textSearch = "",
  byIPFromComment = "",
  filters = "",
  searchFilters = "",
  sorts = "",
  demo = false,
  sso = "your-sso-token"
)

if response.isSome:
  let resp = response.get()
  echo "Found ", resp.comments.len, " comments"
```

### よくある問題

1. **401 認証エラー**: DefaultAPI のリクエストを行う前に HttpClient に `x-api-key` ヘッダーを設定していることを確認してください: `client.headers["x-api-key"] = "your-api-key"`
2. **API クラスの選択ミス**: サーバー側の認証されたリクエストには `api_default`、クライアント側/パブリックなリクエストには `api_public`、モデレーターダッシュボードのリクエストには `api_moderation` を使用してください。