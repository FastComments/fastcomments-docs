---
## Παράμετροι

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |
| id | string | path | Ναι |  |

## Απόκριση

Επιστρέφει: [`GetCachedNotificationCountResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_cached_notification_count_response.py)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα get_cached_notification_count'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_cached_notification_count_response import GetCachedNotificationCountResponse
from client.rest import ApiException
from pprint import pprint

# Ορισμός του host είναι προαιρετικός και προεπιλεγμένος στο https://fastcomments.com
# Δείτε το configuration.py για λίστα όλων των υποστηριζόμενων παραμέτρων διαμόρφωσης.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Ο πελάτης πρέπει να διαμορφώσει τις παραμέτρους αυθεντικοποίησης και εξουσιοδότησης
# σύμφωνα με την πολιτική ασφάλειας του API server.
# Παρακάτω παρέχονται παραδείγματα για κάθε μέθοδο αυθεντικοποίησης — χρησιμοποιήστε το παράδειγμα που
# εξυπηρετεί τη δική σας περίπτωση χρήσης αυθεντικοποίησης.

# Ρυθμίστε την εξουσιοδότηση με API key: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Αποσχολιάστε την παρακάτω γραμμή για να ορίσετε πρόθεμα (π.χ. Bearer) για το API key, αν χρειάζεται
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Εισέλθετε σε context με ένα αντικείμενο του API client
with client.ApiClient(configuration) as api_client:
    # Δημιουργήστε ένα αντικείμενο της κλάσης API
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

---