## Parámetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sí |  |
| urlId | string | query | Sí |  |

## Respuesta

Devuelve: [`GetPageByURLIdAPIResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_page_by_urlid_api_response.py)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de get_page_by_urlid'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_page_by_urlid_api_response import GetPageByURLIdAPIResponse
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
# satisfaga su caso de uso de autenticación.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Enter a context with an instance of the API client
with client.ApiClient(configuration) as api_client:
    # Cree una instancia de la clase API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | 

    try:
        api_response = api_instance.get_page_by_urlid(tenant_id, url_id)
        print("The response of DefaultApi->get_page_by_urlid:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_page_by_urlid: %s\n" % e)
[inline-code-end]