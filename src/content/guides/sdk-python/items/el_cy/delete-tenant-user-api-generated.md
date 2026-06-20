## Παράμετροι

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
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
from client.models.api_empty_response import APIEmptyResponse
from client.rest import ApiException
from pprint import pprint

# Ο ορισμός του host είναι προαιρετικός και έχει προεπιλογή το https://fastcomments.com
# Δείτε το configuration.py για μια λίστα όλων των υποστηριζόμενων παραμέτρων ρύθμισης.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Ο client πρέπει να ρυθμίσει τις παραμέτρους αυθεντικοποίησης και εξουσιοδότησης
# σύμφωνα με την πολιτική ασφαλείας του API server.
# Παρακάτω παρέχονται παραδείγματα για κάθε μέθοδο αυθεντικοποίησης, χρησιμοποιήστε το παράδειγμα που
# ικανοποιεί την περίπτωσή χρήσης αυθεντικοποίησής σας.

# Διαμορφώστε την εξουσιοδότηση με κλειδί API: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Απο-σχολιάστε παρακάτω για να ορίσετε πρόθεμα (π.χ. Bearer) για το κλειδί API, εάν χρειάζεται
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Εισέλθετε σε context με ένα instance του API client
with client.ApiClient(configuration) as api_client:
    # Δημιουργήστε ένα instance της κλάσης API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    delete_comments = 'delete_comments_example' # str |  (optional)
    comment_delete_mode = 'comment_delete_mode_example' # str |  (optional)

    try:
        api_response = api_instance.delete_tenant_user(tenant_id, id, delete_comments=delete_comments, comment_delete_mode=comment_delete_mode)
        print("The response of DefaultApi->delete_tenant_user:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->delete_tenant_user: %s\n" % e)
[inline-code-end]