### Коришћење аутентификованих API‑ја (DefaultAPI)

**Важно:** Аутентификовани крајњи тачке захтевају да ваш API кључ буде постављен у заглављу `x-api-key`.

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_default
import fastcomments/models/model_comment_data

let client = newHttpClient()
client.headers["x-api-key"] = "your-api-key"

# Извршавање аутентификованих API позива.
# Потребни параметри (и тело захтева) су позиционални; опционо
# параметри се прослеђују преко објекта опција операције.
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

### Коришћење јавних API‑ја (PublicAPI)

Јавне крајње тачке не захтевају аутентификацију:

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_public

let client = newHttpClient()

# Извршавање јавних API позива.
# tenantId и urlId су потребни (позиционални); све остало је опционо.
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

### Коришћење модерацијских API‑ја (ModerationAPI)

Модерацијске крајње тачке покрећу контролни панел модератора и аутентификоване су SSO токеном за модератора који делује:

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_moderation

let client = newHttpClient()

# Листање коментара у модерацијском контролном панелу.
# Ова операција нема потребних параметара, тако да је све опционо.
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

### Уобичајени проблеми

1. **401 грешка аутентификације**: Уверите се да сте поставили заглавље `x-api-key` у ваш HttpClient пре него што извршите DefaultAPI захтеве: `client.headers["x-api-key"] = "your-api-key"`
2. **Погрешна API класа**: Користите `api_default` за серверске аутентификоване захтеве, `api_public` за клијентске/јавне захтеве, и `api_moderation` за захтеве контролног панела модератора.