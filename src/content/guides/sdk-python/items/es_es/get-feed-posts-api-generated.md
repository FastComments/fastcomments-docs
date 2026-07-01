req
tenantId
afterId

## Parámetros

| Nombre | Tipo | Ubicación | Requerido | Descripción |
|--------|------|-----------|-----------|-------------|
| tenantId | string | query | Sí |  |
| afterId | string | query | No |  |
| limit | integer | query | No |  |
| tags | array | query | No |  |

## Respuesta

Devuelve: [`GetFeedPostsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_feed_posts_response.py)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo get_feed_posts'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.default_api import GetFeedPostsOptions
from client.models.get_feed_posts_response import GetFeedPostsResponse
from client.rest import ApiException
from pprint import pprint

# Definir el host es opcional y por defecto es https://fastcomments.com
# Ver configuration.py para una lista de todos los parámetros de configuración soportados.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# El cliente debe configurar los parámetros de autenticación y autorización
# de acuerdo con la política de seguridad del servidor API.
# Se proporcionan ejemplos para cada método de autenticación a continuación, usa el ejemplo que
# satisfaga tu caso de uso de autenticación.

# Configurar autorización con clave API: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Descomenta abajo para establecer el prefijo (por ejemplo, Bearer) para la clave API, si es necesario
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Entrar en un contexto con una instancia del cliente API
with client.ApiClient(configuration) as api_client:
    # Crear una instancia de la clase API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    after_id = 'after_id_example' # str |  (opcional)
    limit = 56 # int |  (opcional)
    tags = ['tags_example'] # List[str] |  (opcional)

    try:
        api_response = api_instance.get_feed_posts(tenant_id, GetFeedPostsOptions(after_id=after_id, limit=limit, tags=tags))
        print("The response of DefaultApi->get_feed_posts:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_feed_posts: %s\n" % e)
[inline-code-end]

---