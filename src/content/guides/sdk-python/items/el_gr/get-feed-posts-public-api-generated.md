req
tenantId
afterId

## Παράμετροι

| Όνομα | Type | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ναι |  |
| afterId | string | query | Όχι |  |
| limit | integer | query | Όχι |  |
| tags | array | query | Όχι |  |
| sso | string | query | Όχι |  |
| isCrawler | boolean | query | Όχι |  |
| includeUserInfo | boolean | query | Όχι |  |

## Απόκριση

Επιστρέφει: [`PublicFeedPostsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/public_feed_posts_response.py)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα get_feed_posts_public'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.public_feed_posts_response import PublicFeedPostsResponse
from client.rest import ApiException
from pprint import pprint

# Η ρύθμιση του host είναι προαιρετική και προεπιλεγμένη τιμή είναι το https://fastcomments.com
# Δείτε το configuration.py για λίστα με όλες τις υποστηριζόμενες παραμέτρους ρυθμίσεων.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Εισέλθετε σε ένα context με ένα στιγμιότυπο του API client
with client.ApiClient(configuration) as api_client:
    # Δημιουργήστε ένα στιγμιότυπο της κλάσης API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    after_id = 'after_id_example' # str |  (προαιρετικό)
    limit = 56 # int |  (προαιρετικό)
    tags = ['tags_example'] # List[str] |  (προαιρετικό)
    sso = 'sso_example' # str |  (προαιρετικό)
    is_crawler = True # bool |  (προαιρετικό)
    include_user_info = True # bool |  (προαιρετικό)

    try:
        api_response = api_instance.get_feed_posts_public(tenant_id, after_id=after_id, limit=limit, tags=tags, sso=sso, is_crawler=is_crawler, include_user_info=include_user_info)
        print("The response of PublicApi->get_feed_posts_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_feed_posts_public: %s\n" % e)
[inline-code-end]