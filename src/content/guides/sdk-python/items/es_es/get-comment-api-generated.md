## Parámetros

| Nombre | Tipo | Ubicación | Requerido | Descripción |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sí |  |
| id | string | path | Sí |  |

## Respuesta

Devuelve: [`APIGetCommentResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_get_comment_response.py)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de get_comment'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_get_comment_response import APIGetCommentResponse
from client.rest import ApiException
from pprint import pprint

# Definir el host es opcional y por defecto es https://fastcomments.com
# Consulte configuration.py para la lista de todos los parámetros de configuración admitidos.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# El cliente debe configurar los parámetros de autenticación y autorización
# de acuerdo con la política de seguridad del servidor de la API.
# A continuación se muestran ejemplos para cada método de autenticación; use el que
# se ajuste a su caso de uso de autenticación.

# Configure la autorización por clave de API: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Descomente lo siguiente para configurar el prefijo (por ejemplo Bearer) para la clave de API, si es necesario
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Ingrese a un contexto con una instancia del cliente API
with client.ApiClient(configuration) as api_client:
    # Cree una instancia de la clase API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 

    try:
        api_response = api_instance.get_comment(tenant_id, id)
        print("The response of DefaultApi->get_comment:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_comment: %s\n" % e)
[inline-code-end]