### Использование аутентифицированных API (DefaultAPI)

**Важно:** Аутентифицированные конечные точки требуют, чтобы ваш API-ключ был установлен в заголовке `x-api-key`.

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_default
import fastcomments/models/model_comment_data

let client = newHttpClient()
client.headers["x-api-key"] = "your-api-key"

# Выполняем аутентифицированные вызовы API.
# Обязательные параметры (и тело запроса) передаются позиционно; необязательные
# параметры передаются через объект options операции.
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

### Использование публичных API (PublicAPI)

Публичные конечные точки не требуют аутентификации:

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_public

let client = newHttpClient()

# Выполняем публичные вызовы API.
# tenantId и urlId обязательны (позиционные); всё остальное необязательно.
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

### Использование API модерации (ModerationAPI)

Конечные точки модерации обеспечивают работу панели модератора и аутентифицируются с помощью SSO-токена действующего модератора:

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_moderation

let client = newHttpClient()

# Список комментариев в панели модерации.
# У этой операции нет обязательных параметров, поэтому всё необязательно.
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

### Частые проблемы

1. **Ошибка аутентификации 401**: Убедитесь, что вы установили заголовок `x-api-key` в вашем HttpClient перед выполнением запросов DefaultAPI: `client.headers["x-api-key"] = "your-api-key"`
2. **Неправильный класс API**: Используйте `api_default` для серверных аутентифицированных запросов, `api_public` для клиентских/публичных запросов и `api_moderation` для запросов панели модератора.