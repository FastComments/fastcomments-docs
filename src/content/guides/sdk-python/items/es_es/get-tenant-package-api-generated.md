## Parámetros

| Nombre | Tipo | Ubicación | Requerido | Descripción |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sí |  |
| id | string | path | Sí |  |

## Respuesta

Devuelve: [`GetTenantPackageResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_tenant_package_response.py)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de get_tenant_package'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_tenant_package_response import GetTenantPackageResponse
from client.rest import ApiException
from pprint import pprint

# Definir el host es opcional y por defecto es https://fastcomments.com
# Consulte configuration.py para una lista de todos los parámetros de configuración compatibles.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# El cliente debe configurar los parámetros de autenticación y autorización
# de acuerdo con la política de seguridad del servidor API.
# Se proporcionan ejemplos para cada método de autenticación a continuación; use el ejemplo que
# satisfaga su caso de uso de autenticación.

# Configurar autorización por clave de API: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Descomente abajo para configurar un prefijo (p. ej. Bearer) para la clave de API, si es necesario
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Ingresar a un contexto con una instancia del cliente API
with client.ApiClient(configuration) as api_client:
    # Crear una instancia de la clase API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 

    try:
        api_response = api_instance.get_tenant_package(tenant_id, id)
        print("The response of DefaultApi->get_tenant_package:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_tenant_package: %s\n" % e)
[inline-code-end]