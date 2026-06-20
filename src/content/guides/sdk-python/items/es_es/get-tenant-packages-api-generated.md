## Parámetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sí |  |
| skip | number | query | No |  |

## Respuesta

Devuelve: [`GetTenantPackagesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_tenant_packages_response.py)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de get_tenant_packages'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_tenant_packages_response import GetTenantPackagesResponse
from client.rest import ApiException
from pprint import pprint

# Definir el host es opcional y por defecto es https://fastcomments.com
# Consulta configuration.py para una lista de todos los parámetros de configuración compatibles.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# El cliente debe configurar los parámetros de autenticación y autorización
# de acuerdo con la política de seguridad del servidor API.
# A continuación se proporcionan ejemplos para cada método de autenticación; usa el ejemplo que
# satisfaga tu caso de uso de autenticación.

# Configura la autorización por clave API: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Descomenta abajo para configurar el prefijo (p. ej. Bearer) para la clave API, si es necesario
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Entra en un contexto con una instancia del cliente API
with client.ApiClient(configuration) as api_client:
    # Crea una instancia de la clase API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    skip = 3.4 # float |  (opcional)

    try:
        api_response = api_instance.get_tenant_packages(tenant_id, skip=skip)
        print("The response of DefaultApi->get_tenant_packages:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_tenant_packages: %s\n" % e)
[inline-code-end]