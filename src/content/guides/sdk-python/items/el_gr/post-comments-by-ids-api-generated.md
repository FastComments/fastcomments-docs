## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτούμενο | Περιγραφή |
|------|------|----------|-------------|------------|
| tenantId | string | query | Yes |  |
| sso | string | query | No |  |

## Απόκριση

Επιστρέφει: [`ModerationAPIChildCommentsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/moderation_api_child_comments_response.py)

## Παράδειγμα

[inline-code-attrs-start title = 'post_comments_by_ids Παράδειγμα'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.comments_by_ids_params import CommentsByIdsParams
from client.models.moderation_api_child_comments_response import ModerationAPIChildCommentsResponse
from client.rest import ApiException
from pprint import pprint

# Ο καθορισμός του host είναι προαιρετικός και η προεπιλογή είναι https://fastcomments.com
# Δείτε το configuration.py για μια λίστα με όλες τις υποστηριζόμενες παραμέτρους διαμόρφωσης.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Εισαγωγή ενός context με μια παρουσία του API client
with client.ApiClient(configuration) as api_client:
    # Δημιουργία ενός αντικειμένου της κλάσης API
    api_instance = client.ModerationApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comments_by_ids_params = client.CommentsByIdsParams() # CommentsByIdsParams | 
    sso = 'sso_example' # str |  (optional)

    try:
        api_response = api_instance.post_comments_by_ids(tenant_id, comments_by_ids_params, sso=sso)
        print("The response of ModerationApi->post_comments_by_ids:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->post_comments_by_ids: %s\n" % e)
[inline-code-end]