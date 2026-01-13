### שימוש ב-APIs מאומתים (DefaultAPI)

**חשוב:** נקודות קצה מאומתות דורשות שהמפתח של ה-API יוגדר בכותרת `x-api-key`.

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_default
import fastcomments/models/model_comment_data

let client = newHttpClient()
client.headers["x-api-key"] = "your-api-key"

# בצע קריאות API מאומתות
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

### שימוש ב-APIs ציבוריים (PublicAPI)

נקודות קצה ציבוריות אינן דורשות אימות:

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_public

let client = newHttpClient()

# בצע קריאות API ציבוריות
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

### בעיות נפוצות

1. **שגיאת 401 באימות**: ודא שהגדרת את כותרת `x-api-key` על HttpClient שלך לפני ביצוע בקשות ל-DefaultAPI: `client.headers["x-api-key"] = "your-api-key"`
2. **מחלקת API שגויה**: השתמש ב-`api_default` עבור בקשות מאומתות בצד השרת, וב-`api_public` עבור בקשות בצד הלקוח/ציבוריות.