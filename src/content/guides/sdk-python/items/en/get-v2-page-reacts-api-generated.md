## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes |  |

## Response

Returns: [`GetV2PageReacts`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_v2_page_reacts.py)

## Example

[inline-code-attrs-start title = 'get_v2_page_reacts Example'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_v2_page_reacts import GetV2PageReacts
from client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to https://fastcomments.com
# See configuration.py for a list of all supported configuration parameters.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Enter a context with an instance of the API client
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | 

    try:
        api_response = api_instance.get_v2_page_reacts(tenant_id, url_id)
        print("The response of PublicApi->get_v2_page_reacts:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_v2_page_reacts: %s\n" % e)
[inline-code-end]
