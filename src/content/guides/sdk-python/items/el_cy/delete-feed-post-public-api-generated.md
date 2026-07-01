## Παράμετροι

| Όνομα | Τύπος | Θέση | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| postId | string | path | Yes |  |
| broadcastId | string | query | No |  |
| sso | string | query | No |  |

## Απόκριση

Επιστρέφει: [`DeleteFeedPostPublicResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/delete_feed_post_public_response.py)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα delete_feed_post_public'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import DeleteFeedPostPublicOptions
from client.models.delete_feed_post_public_response import DeleteFeedPostPublicResponse
from client.rest import ApiException
from pprint import pprint

# Η ορισμός του κεντρικού υπολογιστή είναι προαιρετικός και προεπιλεγμένος στη διεύθυνση https://fastcomments.com
# Δείτε το configuration.py για μια λίστα με όλες τις υποστηριζόμενες παραμέτρους διαμόρφωσης.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Εισάγουμε ένα context με μια παρουσία του πελάτη API
with client.ApiClient(configuration) as api_client:
    # Δημιουργούμε μια παρουσία της κλάσης API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    post_id = 'post_id_example' # str | 
    broadcast_id = 'broadcast_id_example' # str |  (optional)
    sso = 'sso_example' # str |  (optional)

    try:
        api_response = api_instance.delete_feed_post_public(tenant_id, post_id, DeleteFeedPostPublicOptions(broadcast_id=broadcast_id, sso=sso))
        print("The response of PublicApi->delete_feed_post_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->delete_feed_post_public: %s\n" % e)
[inline-code-end]