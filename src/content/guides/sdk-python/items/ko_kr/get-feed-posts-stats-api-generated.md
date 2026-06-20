## 매개변수

| 이름 | 유형 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 예 |  |
| postIds | array | query | 예 |  |
| sso | string | query | 아니요 |  |

## 응답

반환: [`FeedPostsStatsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/feed_posts_stats_response.py)

## 예제

[inline-code-attrs-start title = 'get_feed_posts_stats 예제'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.feed_posts_stats_response import FeedPostsStatsResponse
from client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to https://fastcomments.com
# 모든 지원되는 구성 매개변수 목록은 configuration.py를 참조하세요.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Enter a context with an instance of the API client
# API 클라이언트 인스턴스와 함께 컨텍스트를 엽니다
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    # API 클래스의 인스턴스를 생성합니다
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    post_ids = ['post_ids_example'] # List[str] | 
    sso = 'sso_example' # str |  (선택 사항)

    try:
        api_response = api_instance.get_feed_posts_stats(tenant_id, post_ids, sso=sso)
        print("The response of PublicApi->get_feed_posts_stats:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_feed_posts_stats: %s\n" % e)
[inline-code-end]