## 매개변수

| 이름 | 유형 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 예 |  |
| postId | string | path | 예 |  |
| isUndo | boolean | query | 아니오 |  |
| broadcastId | string | query | 아니오 |  |
| sso | string | query | 아니오 |  |

## 응답

반환: [`ReactFeedPostResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/react_feed_post_response.py)

## 예제

[inline-code-attrs-start title = 'react_feed_post_public 예제'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import ReactFeedPostPublicOptions
from client.models.react_body_params import ReactBodyParams
from client.models.react_feed_post_response import ReactFeedPostResponse
from client.rest import ApiException
from pprint import pprint

# 호스트를 정의하는 것은 선택 사항이며 기본값은 https://fastcomments.com 입니다
# 지원되는 모든 구성 매개변수 목록은 configuration.py를 참조하십시오.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# API 클래스의 인스턴스를 생성합니다
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    post_id = 'post_id_example' # str | 
    react_body_params = client.ReactBodyParams() # ReactBodyParams | 
    is_undo = True # bool |  (옵션)
    broadcast_id = 'broadcast_id_example' # str |  (옵션)
    sso = 'sso_example' # str |  (옵션)

    try:
        api_response = api_instance.react_feed_post_public(tenant_id, post_id, react_body_params, ReactFeedPostPublicOptions(is_undo=is_undo, broadcast_id=broadcast_id, sso=sso))
        print("The response of PublicApi->react_feed_post_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->react_feed_post_public: %s\n" % e)
[inline-code-end]