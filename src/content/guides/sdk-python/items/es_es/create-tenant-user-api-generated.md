---
## Parámetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sí |  |

## Respuesta

Devuelve: [`CreateTenantUser200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/create_tenant_user200_response.py)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de create_tenant_user'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.create_tenant_user200_response import CreateTenantUser200Response
from client.models.create_tenant_user_body import CreateTenantUserBody
from client.rest import ApiException
from pprint import pprint

# Definir el host es opcional y por defecto es https://fastcomments.com
# Consulte configuration.py para una lista de todos los parámetros de configuración admitidos.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# El cliente debe configurar los parámetros de autenticación y autorización
# de acuerdo con la política de seguridad del servidor API.
# Se proporcionan ejemplos para cada método de autenticación a continuación; use el ejemplo que
# satisfaga su caso de uso de autenticación.

# Configurar autorización por clave de API: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Descomente abajo para configurar el prefijo (e.g. Bearer) para la clave de API, si es necesario
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Abra un contexto con una instancia del cliente de la API
with client.ApiClient(configuration) as api_client:
    # Crear una instancia de la clase API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    create_tenant_user_body = client.CreateTenantUserBody() # CreateTenantUserBody | 

    try:
        api_response = api_instance.create_tenant_user(tenant_id, create_tenant_user_body)
        print("The response of DefaultApi->create_tenant_user:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->create_tenant_user: %s\n" % e)
[inline-code-end]

---