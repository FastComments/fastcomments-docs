## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | 是 |  |
| postIds | array | query | 是 |  |
| sso | string | query | 否 |  |

## 回應

回傳: [`FeedPostsStatsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/feed_posts_stats_response.py)

## 範例

[inline-code-attrs-start title = 'get_feed_posts_stats 範例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.feed_posts_stats_response import FeedPostsStatsResponse
from client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to https://fastcomments.com
# See configuration.py for a list of all supported configuration parameters.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Enter a context with an instance of the API client
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    post_ids = ['post_ids_example'] # List[str] | 
    sso = 'sso_example' # str |  (可選)

    try:
        api_response = api_instance.get_feed_posts_stats(tenant_id, post_ids, sso=sso)
        print("The response of PublicApi->get_feed_posts_stats:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_feed_posts_stats: %s\n" % e)
[inline-code-end]