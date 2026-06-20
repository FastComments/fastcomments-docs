## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tag | string | path | Ναι |  |
| tenantId | string | query | Όχι |  |

## Απόκριση

Επιστρέφει: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_empty_response.py)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα delete_hash_tag'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_empty_response import APIEmptyResponse
from client.models.delete_hash_tag_request_body import DeleteHashTagRequestBody
from client.rest import ApiException
from pprint import pprint

# Ορισμός του host είναι προαιρετικός και εξ ορισμού είναι https://fastcomments.com
# Δείτε το configuration.py για μια λίστα με όλες τις υποστηριζόμενες παραμέτρους ρύθμισης.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Ο client πρέπει να ρυθμίσει τις παραμέτρους πιστοποίησης και εξουσιοδότησης
# σύμφωνα με την πολιτική ασφάλειας του API server.
# Παρακάτω παρέχονται παραδείγματα για κάθε μέθοδο auth, χρησιμοποιήστε το παράδειγμα που
# ικανοποιεί την περίπτωσή χρήσης σας.

# Ρυθμίστε την εξουσιοδότηση με κλειδί API: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Αποσχολιάστε παρακάτω για να ρυθμίσετε πρόθεμα (π.χ. Bearer) για το API key, αν χρειάζεται
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Εισέλθετε σε ένα context με ένα instance του API client
with client.ApiClient(configuration) as api_client:
    # Δημιουργήστε ένα instance της κλάσης API
    api_instance = client.DefaultApi(api_client)
    tag = 'tag_example' # str | 
    tenant_id = 'tenant_id_example' # str |  (προαιρετικό)
    delete_hash_tag_request_body = client.DeleteHashTagRequestBody() # DeleteHashTagRequestBody |  (προαιρετικό)

    try:
        api_response = api_instance.delete_hash_tag(tag, tenant_id=tenant_id, delete_hash_tag_request_body=delete_hash_tag_request_body)
        print("The response of DefaultApi->delete_hash_tag:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->delete_hash_tag: %s\n" % e)
[inline-code-end]