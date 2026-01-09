### Використання автентифікованих API (DefaultAPI)

**Важливо:** Для автентифікованих кінцевих точок ваш API-ключ повинен бути встановлений у заголовку `x-api-key`.

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_default
import fastcomments/models/model_comment_data

let client = newHttpClient()
client.headers["x-api-key"] = "your-api-key"

# Виконати автентифіковані виклики API
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

### Використання публічних API (PublicAPI)

Публічні кінцеві точки не потребують автентифікації:

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_public

let client = newHttpClient()

# Виконати публічні виклики API
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

### Поширені проблеми

1. **401 помилка автентифікації**: Переконайтеся, що ви встановили заголовок `x-api-key` на вашому HttpClient перед виконанням запитів DefaultAPI: `client.headers["x-api-key"] = "your-api-key"`
2. **Неправильний клас API**: Використовуйте `api_default` для серверних автентифікованих запитів, `api_public` для клієнтських/публічних запитів.