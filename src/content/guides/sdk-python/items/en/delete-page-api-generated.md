## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |

## Response

Returns: [`DeletePageAPIResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/delete_page_api_response.py)

## Example

[inline-code-attrs-start title = 'delete_page Example'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.delete_page_api_response import DeletePageAPIResponse
from client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to https://fastcomments.com
# See configuration.py for a list of all supported configuration parameters.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# The client must configure the authentication and authorization parameters
# in accordance with the API server security policy.
# Examples for each auth method are provided below, use the example that
# satisfies your auth use case.

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

    try:
        api_response = api_instance.delete_page(tenant_id, id)
        print("The response of DefaultApi->delete_page:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->delete_page: %s\n" % e)
[inline-code-end]
