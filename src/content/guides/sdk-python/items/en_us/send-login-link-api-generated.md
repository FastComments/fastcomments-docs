## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |
| redirectURL | string | query | No |  |

## Response

Returns: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/flag_comment_public200_response.py)

## Example

[inline-code-attrs-start title = 'send_login_link Example'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.flag_comment_public200_response import FlagCommentPublic200Response
from client.rest import ApiException
from pprint import pprint

# Defining the host is optional; it defaults to https://fastcomments.com.
# See configuration.py for a list of all supported configuration parameters.
# The client must configure authentication and authorization parameters
# according to the API server's security policy.
# Examples for each auth method are provided below. Use the example that
# matches your auth use case.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Uncomment the line below to set up a prefix (e.g. Bearer) for the API key, if needed
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Enter a context with an instance of the API client
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    redirect_url = 'redirect_url_example' # str |  (optional)

    try:
        api_response = api_instance.send_login_link(tenant_id, id, redirect_url=redirect_url)
        print("The response of DefaultApi->send_login_link:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->send_login_link: %s\n" % e)
[inline-code-end]