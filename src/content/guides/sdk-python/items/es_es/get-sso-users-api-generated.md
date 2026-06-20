## Parámetros

| Nombre | Tipo | Ubicación | Requerido | Descripción |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sí |  |
| skip | integer | query | No |  |

## Respuesta

Devuelve: [`GetSSOUsersResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_sso_users_response.py)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de get_sso_users'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_sso_users_response import GetSSOUsersResponse
from client.rest import ApiException
from pprint import pprint

# Definir el host es opcional y por defecto es https://fastcomments.com
# Consulta configuration.py para una lista de todos los parámetros de configuración compatibles.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# El cliente debe configurar los parámetros de autenticación y autorización
# de acuerdo con la política de seguridad del servidor API.
# Se proporcionan ejemplos para cada método de autenticación más abajo; usa el ejemplo que
# satisfaga tu caso de uso de autenticación.

# Configurar autorización por clave de API: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Descomenta abajo para establecer un prefijo (p. ej. Bearer) para la clave de API, si es necesario
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Entra en un contexto con una instancia del cliente de la API
with client.ApiClient(configuration) as api_client:
    # Crea una instancia de la clase API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    skip = 56 # int |  (opcional)

    try:
        api_response = api_instance.get_sso_users(tenant_id, skip=skip)
        print("The response of DefaultApi->get_sso_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_sso_users: %s\n" % e)
[inline-code-end]