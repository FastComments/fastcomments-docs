## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |

## Response

Returns: [`CreateQuestionConfig200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/create_question_config200_response.py)

## Example

[inline-code-attrs-start title = 'create_question_config Example'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.create_question_config200_response import CreateQuestionConfig200Response
from client.models.create_question_config_body import CreateQuestionConfigBody
from client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to https://fastcomments.com.
# See configuration.py for a list of all supported configuration parameters.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# The client must configure authentication and authorization parameters
# according to the API server's security policy.
# Examples for each auth method are provided below; use the example that
# fits your auth use case.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Uncomment the line below to set up a prefix (e.g. 'Bearer') for the API key, if needed
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Enter a context with an instance of the API client.
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class.
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    create_question_config_body = client.CreateQuestionConfigBody() # CreateQuestionConfigBody | 

    try:
        api_response = api_instance.create_question_config(tenant_id, create_question_config_body)
        print("The response of DefaultApi->create_question_config:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->create_question_config: %s\n" % e)
[inline-code-end]