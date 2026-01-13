## Parámetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sí |  |

## Respuesta

Devuelve: [`CreateTenant200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/create_tenant200_response.py)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de create_tenant'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.create_tenant200_response import CreateTenant200Response
from client.models.create_tenant_body import CreateTenantBody
from client.rest import ApiException
from pprint import pprint

# Definir el host es opcional y por defecto es https://fastcomments.com
# Consulte configuration.py para una lista de todos los parámetros de configuración compatibles.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# El cliente debe configurar los parámetros de autenticación y autorización
# de acuerdo con la política de seguridad del servidor API.
# A continuación se proporcionan ejemplos para cada método de autenticación, use el ejemplo que
# satisface su caso de uso de autenticación.

# Configurar autorización por API key: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Descomente abajo para configurar el prefijo (p. ej. Bearer) para la API key, si es necesario
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Ingrese a un contexto con una instancia del cliente API
with client.ApiClient(configuration) as api_client:
    # Cree una instancia de la clase API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    create_tenant_body = client.CreateTenantBody() # CreateTenantBody | 

    try:
        api_response = api_instance.create_tenant(tenant_id, create_tenant_body)
        print("The response of DefaultApi->create_tenant:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->create_tenant: %s\n" % e)
[inline-code-end]