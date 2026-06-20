req
tenantId
afterId

## 參數

| 名稱 | 類型 | 位置 | 必填 | 說明 |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| afterId | string | query | No |  |
| limit | integer | query | No |  |
| tags | array | query | No |  |
| sso | string | query | No |  |
| isCrawler | boolean | query | No |  |
| includeUserInfo | boolean | query | No |  |

## 回應

回傳: [`PublicFeedPostsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/public_feed_posts_response.py)

## 範例

[inline-code-attrs-start title = 'get_feed_posts_public 範例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.public_feed_posts_response import PublicFeedPostsResponse
from client.rest import ApiException
from pprint import pprint

# 定義主機是可選的，預設為 https://fastcomments.com
# 請參閱 configuration.py，查看所有支援的設定參數清單。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# 使用 API client 的實例進入一個上下文
with client.ApiClient(configuration) as api_client:
    # 建立 API 類別的實例
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    after_id = 'after_id_example' # str |  (選用)
    limit = 56 # int |  (選用)
    tags = ['tags_example'] # List[str] |  (選用)
    sso = 'sso_example' # str |  (選用)
    is_crawler = True # bool |  (選用)
    include_user_info = True # bool |  (選用)

    try:
        api_response = api_instance.get_feed_posts_public(tenant_id, after_id=after_id, limit=limit, tags=tags, sso=sso, is_crawler=is_crawler, include_user_info=include_user_info)
        print("The response of PublicApi->get_feed_posts_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_feed_posts_public: %s\n" % e)
[inline-code-end]