## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | path | हाँ |  |
| commentId | string | path | हाँ |  |
| broadcastId | string | query | हाँ |  |
| sso | string | query | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`ChangeCommentPinStatusResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/change_comment_pin_status_response.py)

## उदाहरण

[inline-code-attrs-start title = 'un_pin_comment उदाहरण'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.change_comment_pin_status_response import ChangeCommentPinStatusResponse
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
    comment_id = 'comment_id_example' # str | 
    broadcast_id = 'broadcast_id_example' # str | 
    sso = 'sso_example' # str |  (optional)

    try:
        api_response = api_instance.un_pin_comment(tenant_id, comment_id, broadcast_id, sso=sso)
        print("The response of PublicApi->un_pin_comment:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->un_pin_comment: %s\n" % e)
[inline-code-end]