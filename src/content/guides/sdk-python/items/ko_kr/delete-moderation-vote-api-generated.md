## 매개변수

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | path | 예 |  |
| voteId | string | path | 예 |  |
| sso | string | query | 아니요 |  |

## 응답

반환: [`VoteDeleteResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/vote_delete_response.py)

## 예제

[inline-code-attrs-start title = 'delete_moderation_vote 예제'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.vote_delete_response import VoteDeleteResponse
from client.rest import ApiException
from pprint import pprint

# 호스트 정의는 선택 사항이며 기본값은 https://fastcomments.com 입니다
# 모든 지원 구성 매개변수 목록은 configuration.py 를 참조하세요.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# API 클라이언트 인스턴스를 사용하여 컨텍스트에 진입합니다
with client.ApiClient(configuration) as api_client:
    # API 클래스의 인스턴스를 생성합니다
    api_instance = client.ModerationApi(api_client)
    comment_id = 'comment_id_example' # str | 
    vote_id = 'vote_id_example' # str | 
    sso = 'sso_example' # str |  (선택 사항)

    try:
        api_response = api_instance.delete_moderation_vote(comment_id, vote_id, sso=sso)
        print("The response of ModerationApi->delete_moderation_vote:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->delete_moderation_vote: %s\n" % e)
[inline-code-end]