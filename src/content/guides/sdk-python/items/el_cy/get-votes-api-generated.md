## Παράμετροι

| Όνομα | Τύπος | Location | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |
| urlId | string | query | Ναι |  |

## Απόκριση

Επιστρέφει: [`GetVotes200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_votes200_response.py)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα get_votes'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_votes200_response import GetVotes200Response
from client.rest import ApiException
from pprint import pprint

# Ορισμός του host είναι προαιρετικός και από προεπιλογή είναι το https://fastcomments.com
# Δείτε το configuration.py για λίστα με όλες τις υποστηριζόμενες παραμέτρους διαμόρφωσης.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Ο client πρέπει να διαμορφώσει τις παραμέτρους αυθεντικοποίησης και εξουσιοδότησης
# σύμφωνα με την πολιτική ασφάλειας του API server.
# Παραδείγματα για κάθε μέθοδο αυθεντικοποίησης παρέχονται πιο κάτω, χρησιμοποιήστε το παράδειγμα που
# καλύπτει την περίπτωσή σας.

# Διαμορφώστε την αυθεντικοποίηση με API key: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Αποσχολιάστε παρακάτω για να ορίσετε πρόθεμα (π.χ. Bearer) για το API key, αν χρειάζεται
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Εισέλθετε σε context με ένα στιγμιότυπο του API client
with client.ApiClient(configuration) as api_client:
    # Δημιουργήστε ένα στιγμιότυπο της κλάσης API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | 

    try:
        api_response = api_instance.get_votes(tenant_id, url_id)
        print("The response of DefaultApi->get_votes:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_votes: %s\n" % e)
[inline-code-end]