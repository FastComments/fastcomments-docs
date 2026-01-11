Agrega documentos agrupándolos (si se proporciona groupBy) y aplicando múltiples operaciones.
Se admiten diferentes operaciones (p. ej. sum, countDistinct, avg, etc.).

## Parámetros

| Nombre | Tipo | Ubicación | Obligatorio | Descripción |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sí |  |
| parentTenantId | string | query | No |  |
| includeStats | boolean | query | No |  |

## Respuesta

Devuelve: [`AggregationResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/aggregation_response.py)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de aggregate'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.aggregation_request import AggregationRequest
from client.models.aggregation_response import AggregationResponse
from client.rest import ApiException
from pprint import pprint

# Definir el host es opcional y por defecto es https://fastcomments.com
# Consulte configuration.py para ver la lista de todos los parámetros de configuración soportados.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# El cliente debe configurar los parámetros de autenticación y autorización
# de acuerdo con la política de seguridad del servidor de la API.
# Se proporcionan ejemplos para cada método de autenticación a continuación, use el ejemplo que
# satisfaga su caso de uso de autenticación.

# Configure la autorización mediante API key: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Descomente abajo para configurar el prefijo (p. ej. Bearer) para la API key, si es necesario
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Entre en un contexto con una instancia del cliente de la API
with client.ApiClient(configuration) as api_client:
    # Cree una instancia de la clase de la API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    aggregation_request = client.AggregationRequest() # AggregationRequest | 
    parent_tenant_id = 'parent_tenant_id_example' # str |  (optional)
    include_stats = True # bool |  (optional)

    try:
        api_response = api_instance.aggregate(tenant_id, aggregation_request, parent_tenant_id=parent_tenant_id, include_stats=include_stats)
        print("The response of DefaultApi->aggregate:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->aggregate: %s\n" % e)
[inline-code-end]