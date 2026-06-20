req
tenantId
afterId

## 매개변수

| 이름 | 형식 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 예 |  |
| afterId | string | query | 아니오 |  |
| limit | integer | query | 아니오 |  |
| tags | array | query | 아니오 |  |
| sso | string | query | 아니오 |  |
| isCrawler | boolean | query | 아니오 |  |
| includeUserInfo | boolean | query | 아니오 |  |

## 응답

반환: [`PublicFeedPostsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/public_feed_posts_response.py)

## 예제

[inline-code-attrs-start title = 'get_feed_posts_public 예제'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.public_feed_posts_response import PublicFeedPostsResponse
from client.rest import ApiException
from pprint import pprint

# 호스트 정의는 선택 사항이며 기본값은 https://fastcomments.com 입니다
# 지원되는 모든 구성 매개변수 목록은 configuration.py를 참조하세요.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# API 클라이언트 인스턴스와 함께 컨텍스트에 진입합니다
with client.ApiClient(configuration) as api_client:
    # API 클래스의 인스턴스를 생성합니다
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    after_id = 'after_id_example' # str |  (선택 사항)
    limit = 56 # int |  (선택 사항)
    tags = ['tags_example'] # List[str] |  (선택 사항)
    sso = 'sso_example' # str |  (선택 사항)
    is_crawler = True # bool |  (선택 사항)
    include_user_info = True # bool |  (선택 사항)

    try:
        api_response = api_instance.get_feed_posts_public(tenant_id, after_id=after_id, limit=limit, tags=tags, sso=sso, is_crawler=is_crawler, include_user_info=include_user_info)
        print("The response of PublicApi->get_feed_posts_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_feed_posts_public: %s\n" % e)
[inline-code-end]