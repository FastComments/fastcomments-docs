## 매개변수

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | 예 |  |
| urlId | string | query | 예 |  |
| broadcastId | string | query | 예 |  |
| sessionId | string | query | 아니요 |  |
| sso | string | query | 아니요 |  |

## 응답

반환: [`CreateCommentPublic200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/create_comment_public200_response.py)

## 예제

[inline-code-attrs-start title = 'create_comment_public 예제'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.comment_data import CommentData
from client.models.create_comment_public200_response import CreateCommentPublic200Response
from client.rest import ApiException
from pprint import pprint

# 호스트 정의는 선택 사항이며 기본값은 https://fastcomments.com 입니다
# 지원되는 모든 구성 매개변수 목록은 configuration.py를 참조하세요.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# API 클라이언트 인스턴스와 함께 컨텍스트를 엽니다
with client.ApiClient(configuration) as api_client:
    # API 클래스의 인스턴스를 생성합니다
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | 
    broadcast_id = 'broadcast_id_example' # str | 
    comment_data = client.CommentData() # CommentData | 
    session_id = 'session_id_example' # str |  (선택 사항)
    sso = 'sso_example' # str |  (선택 사항)

    try:
        api_response = api_instance.create_comment_public(tenant_id, url_id, broadcast_id, comment_data, session_id=session_id, sso=sso)
        print("The response of PublicApi->create_comment_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->create_comment_public: %s\n" % e)
[inline-code-end]