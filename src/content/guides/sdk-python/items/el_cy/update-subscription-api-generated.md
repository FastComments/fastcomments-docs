## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |
| id | string | path | Ναι |  |
| userId | string | query | Όχι |  |

## Απόκριση

Επιστρέφει: [`UpdateSubscriptionAPIResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/update_subscription_api_response.py)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα update_subscription'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.update_api_user_subscription_data import UpdateAPIUserSubscriptionData
from client.models.update_subscription_api_response import UpdateSubscriptionAPIResponse
from client.rest import ApiException
from pprint import pprint

# Ορισμός του host είναι προαιρετικός και προεπιλέγεται σε https://fastcomments.com
# Δείτε το configuration.py για λίστα με όλες τις υποστηριζόμενες παραμέτρους διαμόρφωσης.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Ο πελάτης πρέπει να διαμορφώσει τις παραμέτρους αυθεντικοποίησης και εξουσιοδότησης
# σύμφωνα με την πολιτική ασφαλείας του API server.
# Παραδείγματα για κάθε μέθοδο αυθεντικοποίησης παρέχονται παρακάτω, χρησιμοποιήστε
# το παράδειγμα που καλύπτει την περίπτωσή σας.

# Διαμορφώστε την εξουσιοδότηση με API key: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Απο-σχολιάστε παρακάτω για να ορίσετε πρόθεμα (π.χ. Bearer) για το API key, αν χρειάζεται
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Εισέλθετε σε context με ένα παράδειγμα του API client
with client.ApiClient(configuration) as api_client:
    # Δημιουργήστε ένα παράδειγμα της κλάσης API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    update_api_user_subscription_data = client.UpdateAPIUserSubscriptionData() # UpdateAPIUserSubscriptionData | 
    user_id = 'user_id_example' # str |  (προαιρετικό)

    try:
        api_response = api_instance.update_subscription(tenant_id, id, update_api_user_subscription_data, user_id=user_id)
        print("The response of DefaultApi->update_subscription:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->update_subscription: %s\n" % e)
[inline-code-end]

---