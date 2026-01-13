## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |
| id | string | path | Ναι |  |
| editKey | string | query | Όχι |  |

## Απόκριση

Επιστρέφει: [`DeleteCommentVote200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/delete_comment_vote200_response.py)

## Παράδειγμα

[inline-code-attrs-start title = 'delete_vote Παράδειγμα'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.delete_comment_vote200_response import DeleteCommentVote200Response
from client.rest import ApiException
from pprint import pprint

# Ορισμός του host είναι προαιρετικός και προεπιλεγμένος σε https://fastcomments.com
# Δείτε το configuration.py για μια λίστα με όλες τις υποστηριζόμενες παραμέτρους ρυθμίσεων.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Ο πελάτης πρέπει να ρυθμίσει τις παραμέτρους αυθεντικοποίησης και εξουσιοδότησης
# σύμφωνα με την πολιτική ασφάλειας του API server.
# Παρακάτω παρέχονται παραδείγματα για κάθε μέθοδο αυθεντικοποίησης, χρησιμοποιήστε
# το παράδειγμα που ικανοποιεί την περίπτωση χρήσης αυθεντικοποίησής σας.

# Διαμορφώστε την εξουσιοδότηση με API key: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Αφαιρέστε το σχόλιο παρακάτω για να ορίσετε πρόθεμα (π.χ. Bearer) για το API key, αν χρειάζεται
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Εισέλθετε σε ένα context με ένα στιγμιότυπο του API client
with client.ApiClient(configuration) as api_client:
    # Δημιουργήστε ένα στιγμιότυπο της κλάσης API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    edit_key = 'edit_key_example' # str |  (προαιρετικό)

    try:
        api_response = api_instance.delete_vote(tenant_id, id, edit_key=edit_key)
        print("The response of DefaultApi->delete_vote:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->delete_vote: %s\n" % e)
[inline-code-end]

---