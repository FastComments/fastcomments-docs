## Παράμετροι

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |
| id | string | path | Ναι |  |
| userId | string | query | Όχι |  |

## Απόκριση

Επιστρέφει: [`GetTicket200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_ticket200_response.py)

## Παράδειγμα

[inline-code-attrs-start title = 'get_ticket Παράδειγμα'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_ticket200_response import GetTicket200Response
from client.rest import ApiException
from pprint import pprint

# Η ρύθμιση του host είναι προαιρετική και ως προεπιλογή χρησιμοποιείται το https://fastcomments.com
# Δείτε το configuration.py για λίστα με όλες τις υποστηριζόμενες παραμέτρους διαμόρφωσης.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Ο client πρέπει να διαμορφώσει τις παραμέτρους πιστοποίησης και εξουσιοδότησης
# σύμφωνα με την πολιτική ασφάλειας του API server.
# Παρακάτω παρέχονται παραδείγματα για κάθε μέθοδο auth, χρησιμοποιήστε το παράδειγμα που
# ικανοποιεί την περίπτωση χρήσης auth σας.

# Διαμορφώστε την εξουσιοδότηση με API key: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Αποσχολιάστε παρακάτω για να ρυθμίσετε πρόθεμα (π.χ. Bearer) για το API key, αν χρειάζεται
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Εισέλθετε σε ένα context με ένα instance του API client
with client.ApiClient(configuration) as api_client:
    # Δημιουργήστε ένα instance της κλάσης API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    user_id = 'user_id_example' # str |  (προαιρετικό)

    try:
        api_response = api_instance.get_ticket(tenant_id, id, user_id=user_id)
        print("The response of DefaultApi->get_ticket:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_ticket: %s\n" % e)
[inline-code-end]