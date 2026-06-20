## Παράμετροι

| Όνομα | Τύπος | Location | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |
| userId | string | query | Ναι |  |
| id | string | path | Ναι |  |

## Απόκριση

Επιστρέφει: [`ChangeTicketStateResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/change_ticket_state_response.py)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα change_ticket_state'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.change_ticket_state_body import ChangeTicketStateBody
from client.models.change_ticket_state_response import ChangeTicketStateResponse
from client.rest import ApiException
from pprint import pprint

# Ο καθορισμός του host είναι προαιρετικός και προεπιλέγεται στο https://fastcomments.com
# Δείτε το configuration.py για μια λίστα με όλες τις υποστηριζόμενες παραμέτρους διαμόρφωσης.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Ο client πρέπει να διαμορφώσει τις παραμέτρους αυθεντικοποίησης και εξουσιοδότησης
# σύμφωνα με την πολιτική ασφαλείας του API server.
# Παρέχονται παραδείγματα για κάθε μέθοδο αυθεντικοποίησης παρακάτω, χρησιμοποιήστε το παράδειγμα που
# ικανοποιεί την περίπτωση χρήσης αυθεντικοποίησής σας.

# Διαμορφώστε την εξουσιοδότηση με API key: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Απο-σχολιάστε παρακάτω για να ορίσετε πρόθεμα (π.χ. Bearer) για το API key, αν χρειάζεται
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Εισέλθετε σε ένα context με ένα instance του API client
with client.ApiClient(configuration) as api_client:
    # Δημιουργήστε ένα instance της κλάσης API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    user_id = 'user_id_example' # str | 
    id = 'id_example' # str | 
    change_ticket_state_body = client.ChangeTicketStateBody() # ChangeTicketStateBody | 

    try:
        api_response = api_instance.change_ticket_state(tenant_id, user_id, id, change_ticket_state_body)
        print("The response of DefaultApi->change_ticket_state:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->change_ticket_state: %s\n" % e)
[inline-code-end]