### Uso de APIs autenticadas (DefaultAPI)

**Importante:** Los puntos finales autenticados requieren que tu clave API se establezca en el encabezado `x-api-key`.

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_default
import fastcomments/models/model_comment_data

let client = newHttpClient()
client.headers["x-api-key"] = "your-api-key"

# Realizar llamadas a la API autenticada.
# Los parámetros requeridos (y el cuerpo de la solicitud) son posicionales; los opcionales
# los parámetros se pasan mediante el objeto de opciones de la operación.
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

### Uso de APIs públicas (PublicAPI)

Los puntos finales públicos no requieren autenticación:

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_public

let client = newHttpClient()

# Realizar llamadas a la API pública.
# tenantId y urlId son requeridos (posicionales); todo lo demás es opcional.
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

### Uso de APIs de moderación (ModerationAPI)

Los puntos finales de moderación impulsan el panel de moderador y se autentican con un token SSO para el moderador activo:

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_moderation

let client = newHttpClient()

# Listar comentarios en el panel de moderación.
# Esta operación no tiene parámetros requeridos, por lo que todo es opcional.
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

### Problemas comunes

1. **Error de autenticación 401**: Asegúrate de establecer el encabezado `x-api-key` en tu HttpClient antes de realizar solicitudes a DefaultAPI: `client.headers["x-api-key"] = "your-api-key"`
2. **Clase API incorrecta**: Usa `api_default` para solicitudes autenticadas del lado del servidor, `api_public` para solicitudes del lado del cliente/públicas, y `api_moderation` para solicitudes del panel de moderador.