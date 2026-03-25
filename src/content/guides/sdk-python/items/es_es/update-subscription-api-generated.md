## Parámetros

| Nombre | Tipo | Ubicación | Obligatorio | Descripción |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sí |  |
| id | string | path | Sí |  |
| userId | string | query | No |  |

## Respuesta

Devuelve: [`UpdateSubscriptionAPIResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/update_subscription_api_response.py)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de update_subscription'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.update_api_user_subscription_data import UpdateAPIUserSubscriptionData
from client.models.update_subscription_api_response import UpdateSubscriptionAPIResponse
from client.rest import ApiException
from pprint import pprint

# Definir el host es opcional y por defecto es https://fastcomments.com
# Consulte configuration.py para ver la lista de todos los parámetros de configuración compatibles.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# El cliente debe configurar los parámetros de autenticación y autorización
# de acuerdo con la política de seguridad del servidor API.
# A continuación se proporcionan ejemplos para cada método de autenticación, use el ejemplo que
# satisfaga su caso de uso de autenticación.

# Configurar la autorización con clave API: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Descomente lo siguiente para configurar un prefijo (p. ej., Bearer) para la clave API, si es necesario
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Entre en un contexto con una instancia del cliente API
with client.ApiClient(configuration) as api_client:
    # Cree una instancia de la clase API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    update_api_user_subscription_data = client.UpdateAPIUserSubscriptionData() # UpdateAPIUserSubscriptionData | 
    user_id = 'user_id_example' # str |  (opcional)

    try:
        api_response = api_instance.update_subscription(tenant_id, id, update_api_user_subscription_data, user_id=user_id)
        print("The response of DefaultApi->update_subscription:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->update_subscription: %s\n" % e)
[inline-code-end]