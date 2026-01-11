## Parámetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sí |  |
| id | string | path | Sí |  |
| deleteComments | boolean | query | No |  |
| commentDeleteMode | string | query | No |  |

## Respuesta

Devuelve: [`DeleteSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/delete_sso_user_api_response.py)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de delete_sso_user'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.delete_sso_user_api_response import DeleteSSOUserAPIResponse
from client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to https://fastcomments.com
# Consulte configuration.py para una lista de todos los parámetros de configuración compatibles.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# The client must configure the authentication and authorization parameters
# in accordance with the API server security policy.
# A continuación se proporcionan ejemplos para cada método de autenticación, use el ejemplo que
# satisfaga su caso de uso de autenticación.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Enter a context with an instance of the API client
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    delete_comments = True # bool |  (opcional)
    comment_delete_mode = 'comment_delete_mode_example' # str |  (opcional)

    try:
        api_response = api_instance.delete_sso_user(tenant_id, id, delete_comments=delete_comments, comment_delete_mode=comment_delete_mode)
        print("The response of DefaultApi->delete_sso_user:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->delete_sso_user: %s\n" % e)
[inline-code-end]

---