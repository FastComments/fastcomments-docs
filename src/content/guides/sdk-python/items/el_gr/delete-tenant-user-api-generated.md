## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|------------|-----------|
| tenantId | string | query | Ναι |  |
| id | string | path | Ναι |  |
| deleteComments | string | query | Όχι |  |
| commentDeleteMode | string | query | Όχι |  |

## Απόκριση

Επιστρέφει: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_empty_response.py)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα delete_tenant_user'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.default_api import DeleteTenantUserOptions
from client.models.api_empty_response import APIEmptyResponse
from client.rest import ApiException
from pprint import pprint

# Ο ορισμός του κεντρικού υπολογιστή είναι προαιρετικός και προεπιλεγμένος στο https://fastcomments.com
# Δείτε το configuration.py για τη λίστα όλων των υποστηριζόμενων παραμέτρων ρυθμίσεων.
# Ο πελάτης πρέπει να ρυθμίσει τις παραμέτρους ταυτοποίησης και εξουσιοδότησης
# σύμφωνα με την πολιτική ασφαλείας του διακομιστή API.
# Παρέχονται παραδείγματα για κάθε μέθοδο ταυτοποίησης, χρησιμοποιήστε το παράδειγμα που
# εξυπηρετεί το σενάριο ταυτοποίησής σας.

# Ρύθμιση εξουσιοδότησης με κλειδί API: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Ξεσχολιάστε παρακάτω για να ορίσετε πρόθεμα (π.χ. Bearer) για το κλειδί API, εάν χρειάζεται
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Εισαγωγή σε ένα περιβάλλον με μια παρουσία του πελάτη API
with client.ApiClient(configuration) as api_client:
    # Δημιουργία μιας παρουσίασης της κλάσης API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    delete_comments = 'delete_comments_example' # str |  (προαιρετικό)
    comment_delete_mode = 'comment_delete_mode_example' # str |  (προαιρετικό)

    try:
        api_response = api_instance.delete_tenant_user(tenant_id, id, DeleteTenantUserOptions(delete_comments=delete_comments, comment_delete_mode=comment_delete_mode))
        print("The response of DefaultApi->delete_tenant_user:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->delete_tenant_user: %s\n" % e)
[inline-code-end]