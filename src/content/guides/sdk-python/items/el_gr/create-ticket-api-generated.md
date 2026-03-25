## Παράμετροι

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |
| userId | string | query | Ναι |  |

## Απάντηση

Επιστρέφει: [`CreateTicket200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/create_ticket200_response.py)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα create_ticket'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.create_ticket200_response import CreateTicket200Response
from client.models.create_ticket_body import CreateTicketBody
from client.rest import ApiException
from pprint import pprint

# Ορισμός του host είναι προαιρετικός και η προεπιλεγμένη τιμή είναι https://fastcomments.com
# Δείτε το configuration.py για λίστα όλων των υποστηριζόμενων παραμέτρων διαμόρφωσης.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Ο πελάτης πρέπει να διαμορφώσει τις παραμέτρους ταυτοποίησης και εξουσιοδότησης
# σύμφωνα με την πολιτική ασφάλειας του διακομιστή API.
# Παραδείγματα για κάθε μέθοδο πιστοποίησης παρέχονται παρακάτω, χρησιμοποιήστε το παράδειγμα που
# ικανοποιεί την περίπτωση χρήσης της πιστοποίησής σας.

# Διαμορφώστε την εξουσιοδότηση με API key: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Αποσχολιάστε το παρακάτω για να ορίσετε πρόθεμα (π.χ. Bearer) για το API key, αν χρειάζεται
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Εισέλθετε σε ένα πλαίσιο (context) με ένα παράδειγμα του πελάτη API
with client.ApiClient(configuration) as api_client:
    # Δημιουργήστε ένα στιγμιότυπο της κλάσης API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    user_id = 'user_id_example' # str | 
    create_ticket_body = client.CreateTicketBody() # CreateTicketBody | 

    try:
        api_response = api_instance.create_ticket(tenant_id, user_id, create_ticket_body)
        print("The response of DefaultApi->create_ticket:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->create_ticket: %s\n" % e)
[inline-code-end]