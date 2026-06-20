## Παράμετροι

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |
| updateComments | string | query | No |  |

## Απόκριση

Επιστρέφει: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_empty_response.py)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα update_tenant_user'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_empty_response import APIEmptyResponse
from client.models.update_tenant_user_body import UpdateTenantUserBody
from client.rest import ApiException
from pprint import pprint

# Ορισμός του host είναι προαιρετικός και έχει ως προεπιλογή το https://fastcomments.com
# Δείτε το configuration.py για λίστα με όλες τις υποστηριζόμενες παραμέτρους ρυθμίσεων.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Ο πελάτης πρέπει να ρυθμίσει τις παραμέτρους αυθεντικοποίησης και εξουσιοδότησης
# σύμφωνα με την πολιτική ασφαλείας του API server.
# Παρακάτω παρέχονται παραδείγματα για κάθε μέθοδο auth, χρησιμοποιήστε το παράδειγμα που
# ικανοποιεί τη χρήση σας.

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
    update_tenant_user_body = client.UpdateTenantUserBody() # UpdateTenantUserBody | 
    update_comments = 'update_comments_example' # str |  (optional)

    try:
        api_response = api_instance.update_tenant_user(tenant_id, id, update_tenant_user_body, update_comments=update_comments)
        print("The response of DefaultApi->update_tenant_user:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->update_tenant_user: %s\n" % e)
[inline-code-end]