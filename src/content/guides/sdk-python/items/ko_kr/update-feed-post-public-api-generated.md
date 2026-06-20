## 매개변수

| 이름 | 타입 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 예 |  |
| postId | string | path | 예 |  |
| broadcastId | string | query | 아니오 |  |
| sso | string | query | 아니오 |  |

## 응답

반환: [`CreateFeedPostResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/create_feed_post_response.py)

## 예제

[inline-code-attrs-start title = 'update_feed_post_public 예제'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.create_feed_post_response import CreateFeedPostResponse
from client.models.update_feed_post_params import UpdateFeedPostParams
from client.rest import ApiException
from pprint import pprint

# 호스트를 정의하는 것은 선택 사항이며 기본값은 https://fastcomments.com 입니다
# See configuration.py for a list of all supported configuration parameters.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# API 클라이언트 인스턴스로 컨텍스트를 엽니다
with client.ApiClient(configuration) as api_client:
    # API 클래스의 인스턴스를 생성합니다
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    post_id = 'post_id_example' # str | 
    update_feed_post_params = client.UpdateFeedPostParams() # UpdateFeedPostParams | 
    broadcast_id = 'broadcast_id_example' # str |  (선택 사항)
    sso = 'sso_example' # str |  (선택 사항)

    try:
        api_response = api_instance.update_feed_post_public(tenant_id, post_id, update_feed_post_params, broadcast_id=broadcast_id, sso=sso)
        print("The response of PublicApi->update_feed_post_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->update_feed_post_public: %s\n" % e)
[inline-code-end]