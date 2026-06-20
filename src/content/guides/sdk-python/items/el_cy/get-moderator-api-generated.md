## Παράμετροι

| Όνομα | Type | Location | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |
| id | string | path | Ναι |  |

## Απόκριση

Επιστρέφει: [`GetModeratorResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_moderator_response.py)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα get_moderator'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_moderator_response import GetModeratorResponse
from client.rest import ApiException
from pprint import pprint

# Ορισμός του host είναι προαιρετικός και έχει προεπιλογή https://fastcomments.com
# Δείτε το configuration.py για λίστα με όλες τις υποστηριζόμενες παραμέτρους διαμόρφωσης.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Ο client πρέπει να διαμορφώσει τις παραμέτρους authentication και authorization
# σύμφωνα με την πολιτική ασφαλείας του API server.
# Παρακάτω παρέχονται παραδείγματα για κάθε μέθοδο auth, χρησιμοποιήστε το παράδειγμα που
# ικανοποιεί τη δική σας περίπτωση χρήσης auth.

# Διαμόρφωση εξουσιοδότησης με API key: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Αφαιρέστε το σχόλιο παρακάτω για να ορίσετε prefix (π.χ. Bearer) για το API key, αν χρειάζεται
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Εισέλθετε σε context με ένα instance του API client
with client.ApiClient(configuration) as api_client:
    # Δημιουργήστε ένα instance της κλάσης API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 

    try:
        api_response = api_instance.get_moderator(tenant_id, id)
        print("The response of DefaultApi->get_moderator:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_moderator: %s\n" % e)
[inline-code-end]