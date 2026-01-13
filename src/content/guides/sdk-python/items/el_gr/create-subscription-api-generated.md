## Παράμετροι

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |

## Απόκριση

Επιστρέφει: [`CreateSubscriptionAPIResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/create_subscription_api_response.py)

## Παράδειγμα

[inline-code-attrs-start title = 'create_subscription Παράδειγμα'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.create_api_user_subscription_data import CreateAPIUserSubscriptionData
from client.models.create_subscription_api_response import CreateSubscriptionAPIResponse
from client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to https://fastcomments.com
# Δείτε το configuration.py για λίστα με όλες τις υποστηριζόμενες παραμέτρους ρύθμισης.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# The client must configure the authentication and authorization parameters
# σύμφωνα με την πολιτική ασφάλειας του API server.
# Παρακάτω παρέχονται παραδείγματα για κάθε μέθοδο auth — χρησιμοποιήστε το παράδειγμα που
# καλύπτει την περίπτωση χρήσης auth σας.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# Αποσχολιάστε παρακάτω για να ρυθμίσετε prefix (π.χ. Bearer) για το API key, αν χρειάζεται
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Enter a context with an instance of the API client
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    create_api_user_subscription_data = client.CreateAPIUserSubscriptionData() # CreateAPIUserSubscriptionData | 

    try:
        api_response = api_instance.create_subscription(tenant_id, create_api_user_subscription_data)
        print("The response of DefaultApi->create_subscription:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->create_subscription: %s\n" % e)
[inline-code-end]

---