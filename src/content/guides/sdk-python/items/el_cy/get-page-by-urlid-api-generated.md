## Παράμετροι

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |
| urlId | string | query | Ναι |  |

## Απόκριση

Επιστρέφει: [`GetPageByURLIdAPIResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_page_by_urlid_api_response.py)

## Παράδειγμα

[inline-code-attrs-start title = 'get_page_by_urlid Παράδειγμα'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_page_by_urlid_api_response import GetPageByURLIdAPIResponse
from client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to https://fastcomments.com
# Δείτε το configuration.py για λίστα με όλες τις υποστηριζόμενες παραμέτρους διαμόρφωσης.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# The client must configure the authentication and authorization parameters
# σύμφωνα με την πολιτική ασφαλείας του API server.
# Παραδείγματα για κάθε μέθοδο αυθεντικοποίησης παρέχονται παρακάτω, χρησιμοποιήστε το παράδειγμα που
# ικανοποιεί την περίπτωση χρήσης αυθεντικοποίησής σας.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Enter a context with an instance of the API client
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
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