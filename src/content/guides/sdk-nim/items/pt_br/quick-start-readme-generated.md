### Usando APIs Autenticadas (DefaultAPI)

**Importante:** Endpoints autenticados requerem que sua chave de API seja definida no cabeçalho `x-api-key`.

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_default
import fastcomments/models/model_comment_data

let client = newHttpClient()
client.headers["x-api-key"] = "your-api-key"

# Faça chamadas de API autenticadas.
# Parâmetros obrigatórios (e o corpo da requisição) são posicionais; opcionais
# parâmetros são passados via o objeto de opções da operação.
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

### Usando APIs Públicas (PublicAPI)

Endpoints públicos não requerem autenticação:

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_public

let client = newHttpClient()

# Faça chamadas de API públicas.
# tenantId e urlId são obrigatórios (posicionais); todo o resto é opcional.
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

### Usando APIs de Moderação (ModerationAPI)

Endpoints de moderação alimentam o painel do moderador e são autenticados com um token SSO para o moderador em atuação:

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_moderation

let client = newHttpClient()

# Listar comentários no painel de moderação.
# Esta operação não tem parâmetros obrigatórios, portanto tudo é opcional.
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

### Problemas Comuns

1. **Erro de autenticação 401**: Certifique-se de definir o cabeçalho `x-api-key` no seu HttpClient antes de fazer requisições DefaultAPI: `client.headers["x-api-key"] = "your-api-key"`
2. **Classe de API incorreta**: Use `api_default` para requisições autenticadas do lado do servidor, `api_public` para requisições do lado do cliente/públicas, e `api_moderation` para requisições do painel de moderador.