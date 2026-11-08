## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | path | हाँ |  |
| urlId | string | query | हाँ |  |
| title | string | query | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`CreateV1PageReact`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/create_v1_page_react.py)

## उदाहरण

[inline-code-attrs-start title = 'create_v1_page_react उदाहरण'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.create_v1_page_react import CreateV1PageReact
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
    title = 'title_example' # str |  (optional)

    try:
        api_response = api_instance.create_v1_page_react(tenant_id, url_id, title=title)
        print("The response of PublicApi->create_v1_page_react:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->create_v1_page_react: %s\n" % e)
[inline-code-end]