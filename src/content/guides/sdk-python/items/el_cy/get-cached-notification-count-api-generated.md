## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |

## Απόκριση

Επιστρέφει: [`GetCachedNotificationCountResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_cached_notification_count_response.py)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα get_cached_notification_count'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_cached_notification_count_response import GetCachedNotificationCountResponse
from client.rest import ApiException
from pprint import pprint

# Ο ορισμός του host είναι προαιρετικός και από προεπιλογή είναι το https://fastcomments.com
# Δείτε το configuration.py για λίστα με όλες τις υποστηριζόμενες παραμέτρους ρυθμίσεων.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# O client πρέπει να ρυθμίσει τις παραμέτρους επαλήθευσης και εξουσιοδότησης
# σύμφωνα με την πολιτική ασφαλείας του API server.
# Τα παραδείγματα για κάθε μέθοδο επαλήθευσης δίνονται παρακάτω — χρησιμοποιήστε
# το παράδειγμα που ταιριάζει στην περίπτωση χρήσης σας.

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
        api_response = api_instance.get_cached_notification_count(tenant_id, id)
        print("The response of DefaultApi->get_cached_notification_count:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_cached_notification_count: %s\n" % e)
[inline-code-end]