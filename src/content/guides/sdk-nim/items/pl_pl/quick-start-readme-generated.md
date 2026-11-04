### Korzystanie z uwierzytelnianych API (DefaultAPI)

**Ważne:** Uwierzytelniane endpointy wymagają ustawienia klucza API w nagłówku `x-api-key`.

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_default
import fastcomments/models/model_comment_data

let client = newHttpClient()
client.headers["x-api-key"] = "your-api-key"

# Wykonaj uwierzytelnione wywołania API.
# Parametry wymagane (oraz treść żądania) są pozycyjne; parametry
# opcjonalne przekazuje się przez obiekt opcji danej operacji.
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

### Korzystanie z publicznych API (PublicAPI)

Publiczne endpointy nie wymagają uwierzytelnienia:

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_public

let client = newHttpClient()

# Wykonaj publiczne wywołania API.
# tenantId i urlId są wymagane (pozycyjne); wszystko inne jest opcjonalne.
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

### Korzystanie z API moderacji (ModerationAPI)

Endpointy moderacji obsługują panel moderatora i są uwierzytelniane tokenem SSO działającego moderatora:

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_moderation

let client = newHttpClient()

# Wyświetl komentarze w panelu moderacji.
# Ta operacja nie ma wymaganych parametrów, więc wszystko jest opcjonalne.
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

### Częste problemy

1. **Błąd uwierzytelniania 401**: Upewnij się, że ustawiłeś nagłówek `x-api-key` w swoim HttpClient przed wykonywaniem żądań DefaultAPI: `client.headers["x-api-key"] = "your-api-key"`
2. **Zła klasa API**: Użyj `api_default` dla uwierzytelnianych żądań po stronie serwera, `api_public` dla żądań po stronie klienta/publicznych oraz `api_moderation` dla żądań panelu moderatora.