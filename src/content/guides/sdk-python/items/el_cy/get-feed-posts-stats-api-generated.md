## Παράμετροι

| Όνομα | Τύπος | Θέση | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ναι |  |
| postIds | array | query | Ναι |  |
| sso | string | query | Όχι |  |

## Απόκριση

Επιστρέφει: [`FeedPostsStatsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/feed_posts_stats_response.py)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα get_feed_posts_stats'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.feed_posts_stats_response import FeedPostsStatsResponse
from client.rest import ApiException
from pprint import pprint

# Ορισμός του host είναι προαιρετικός και προεπιλογή είναι το https://fastcomments.com
# Δείτε το configuration.py για μια λίστα με όλες τις υποστηριζόμενες παραμέτρους ρύθμισης.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Ανοίξτε ένα context με ένα instance του API client
with client.ApiClient(configuration) as api_client:
    # Δημιουργήστε ένα instance της κλάσης API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    post_ids = ['post_ids_example'] # List[str] | 
    sso = 'sso_example' # str |  (optional)

    try:
        api_response = api_instance.get_feed_posts_stats(tenant_id, post_ids, sso=sso)
        print("The response of PublicApi->get_feed_posts_stats:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_feed_posts_stats: %s\n" % e)
[inline-code-end]