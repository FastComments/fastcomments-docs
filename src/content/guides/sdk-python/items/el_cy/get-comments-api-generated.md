## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |
| page | integer | query | Όχι |  |
| limit | integer | query | Όχι |  |
| skip | integer | query | Όχι |  |
| asTree | boolean | query | Όχι |  |
| skipChildren | integer | query | Όχι |  |
| limitChildren | integer | query | Όχι |  |
| maxTreeDepth | integer | query | Όχι |  |
| urlId | string | query | Όχι |  |
| userId | string | query | Όχι |  |
| anonUserId | string | query | Όχι |  |
| contextUserId | string | query | Όχι |  |
| hashTag | string | query | Όχι |  |
| parentId | string | query | Όχι |  |
| direction | string | query | Όχι |  |

## Απόκριση

Επιστρέφει: [`GetComments200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_comments200_response.py)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα get_comments'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_comments200_response import GetComments200Response
from client.models.sort_directions import SortDirections
from client.rest import ApiException
from pprint import pprint

# Ορισμός του host είναι προαιρετικός και προεπιλεγμένο είναι το https://fastcomments.com
# Δείτε το configuration.py για λίστα με όλες τις υποστηριζόμενες παραμέτρους ρύθμισης.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Ο client πρέπει να διαμορφώσει τις παραμέτρους αυθεντικοποίησης και εξουσιοδότησης
# σύμφωνα με την πολιτική ασφαλείας του διακομιστή API.
# Παραδείγματα για κάθε μέθοδο αυθεντικοποίησης παρέχονται παρακάτω, χρησιμοποιήστε το παράδειγμα που
# ταιριάζει στην περίπτωση χρήσης σας.

# Διαμορφώστε την αυθεντικοποίηση με API key: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Αποσχολιάστε παρακάτω για να ορίσετε πρόθεμα (π.χ. Bearer) για το API key, αν χρειάζεται
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Εισέλθετε σε context με ένα instance του API client
with client.ApiClient(configuration) as api_client:
    # Δημιουργήστε ένα instance της κλάσης API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    page = 56 # int |  (προαιρετικό)
    limit = 56 # int |  (προαιρετικό)
    skip = 56 # int |  (προαιρετικό)
    as_tree = True # bool |  (προαιρετικό)
    skip_children = 56 # int |  (προαιρετικό)
    limit_children = 56 # int |  (προαιρετικό)
    max_tree_depth = 56 # int |  (προαιρετικό)
    url_id = 'url_id_example' # str |  (προαιρετικό)
    user_id = 'user_id_example' # str |  (προαιρετικό)
    anon_user_id = 'anon_user_id_example' # str |  (προαιρετικό)
    context_user_id = 'context_user_id_example' # str |  (προαιρετικό)
    hash_tag = 'hash_tag_example' # str |  (προαιρετικό)
    parent_id = 'parent_id_example' # str |  (προαιρετικό)
    direction = client.SortDirections() # SortDirections |  (προαιρετικό)

    try:
        api_response = api_instance.get_comments(tenant_id, page=page, limit=limit, skip=skip, as_tree=as_tree, skip_children=skip_children, limit_children=limit_children, max_tree_depth=max_tree_depth, url_id=url_id, user_id=user_id, anon_user_id=anon_user_id, context_user_id=context_user_id, hash_tag=hash_tag, parent_id=parent_id, direction=direction)
        print("The response of DefaultApi->get_comments:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_comments: %s\n" % e)
[inline-code-end]