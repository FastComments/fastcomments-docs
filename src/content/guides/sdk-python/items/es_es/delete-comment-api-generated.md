## Parámetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sí |  |
| id | string | path | Sí |  |
| contextUserId | string | query | No |  |
| isLive | boolean | query | No |  |

## Respuesta

Devuelve: [`DeleteCommentResult`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/delete_comment_result.py)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de delete_comment'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.delete_comment_result import DeleteCommentResult
from client.rest import ApiException
from pprint import pprint

# Definir el host es opcional y por defecto es https://fastcomments.com
# Consulte configuration.py para una lista de todos los parámetros de configuración compatibles.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# El cliente debe configurar los parámetros de autenticación y autorización
# de acuerdo con la política de seguridad del servidor de la API.
# Se proporcionan ejemplos para cada método de autenticación a continuación; use el ejemplo que
# satisfaga su caso de uso de autenticación.

# Configurar autorización por clave API: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Descomente abajo para configurar un prefijo (p. ej. Bearer) para la clave API, si es necesario
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Ingrese a un contexto con una instancia del cliente de la API
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    context_user_id = 'context_user_id_example' # str |  (opcional)
    is_live = True # bool |  (opcional)

    try:
        api_response = api_instance.delete_comment(tenant_id, id, context_user_id=context_user_id, is_live=is_live)
        print("The response of DefaultApi->delete_comment:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->delete_comment: %s\n" % e)
[inline-code-end]

---