## Parámetros

| Nombre | Tipo | Ubicación | Requerido | Descripción |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sí |  |
| id | string | path | Sí |  |
| userId | string | query | No |  |

## Respuesta

Devuelve: [`GetTicket200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_ticket200_response.py)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de get_ticket'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_ticket200_response import GetTicket200Response
from client.rest import ApiException
from pprint import pprint

# Definir el host es opcional y por defecto es https://fastcomments.com
# Consulte configuration.py para obtener una lista de todos los parámetros de configuración compatibles.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# El cliente debe configurar los parámetros de autenticación y autorización
# de acuerdo con la política de seguridad del servidor API.
# Se proporcionan ejemplos para cada método de autenticación a continuación; utilice el ejemplo que
# satisfaga su caso de uso de autenticación.

# Configurar la autorización mediante clave API: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Descomente a continuación para establecer un prefijo (p. ej., Bearer) para la clave API, si es necesario
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Entre en un contexto con una instancia del cliente API
with client.ApiClient(configuration) as api_client:
    # Cree una instancia de la clase API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    user_id = 'user_id_example' # str |  (opcional)

    try:
        api_response = api_instance.get_ticket(tenant_id, id, user_id=user_id)
        print("The response of DefaultApi->get_ticket:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_ticket: %s\n" % e)
[inline-code-end]