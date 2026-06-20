## 매개변수

| 이름 | 타입 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 예 |  |
| commentId | string | path | 예 |  |
| voteId | string | path | 예 |  |
| urlId | string | query | 예 |  |
| broadcastId | string | query | 예 |  |
| editKey | string | query | 아니요 |  |
| sso | string | query | 아니요 |  |

## 응답

반환: [`VoteDeleteResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/vote_delete_response.py)

## 예제

[inline-code-attrs-start title = 'delete_comment_vote 예제'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.vote_delete_response import VoteDeleteResponse
from client.rest import ApiException
from pprint import pprint

# 호스트 정의는 선택 사항이며 기본값은 https://fastcomments.com 입니다
# 지원되는 모든 구성 매개변수 목록은 configuration.py를 참조하세요.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# API 클라이언트 인스턴스로 컨텍스트를 엽니다
with client.ApiClient(configuration) as api_client:
    # API 클래스의 인스턴스를 생성합니다
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_id = 'comment_id_example' # str | 
    vote_id = 'vote_id_example' # str | 
    url_id = 'url_id_example' # str | 
    broadcast_id = 'broadcast_id_example' # str | 
    edit_key = 'edit_key_example' # str |  (optional)
    sso = 'sso_example' # str |  (optional)

    try:
        api_response = api_instance.delete_comment_vote(tenant_id, comment_id, vote_id, url_id, broadcast_id, edit_key=edit_key, sso=sso)
        print("The response of PublicApi->delete_comment_vote:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->delete_comment_vote: %s\n" % e)
[inline-code-end]