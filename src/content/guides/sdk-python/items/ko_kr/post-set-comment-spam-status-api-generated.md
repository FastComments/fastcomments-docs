## 매개변수

| 이름 | 유형 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| commentId | string | path | Yes |  |
| spam | boolean | query | No |  |
| permNotSpam | boolean | query | No |  |
| broadcastId | string | query | No |  |
| sso | string | query | No |  |

## 응답

반환: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_empty_response.py)

## 예시

[inline-code-attrs-start title = 'post_set_comment_spam_status 예시'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.moderation_api import PostSetCommentSpamStatusOptions
from client.models.api_empty_response import APIEmptyResponse
from client.rest import ApiException
from pprint import pprint

# 호스트 정의는 선택 사항이며 기본값은 https://fastcomments.com 입니다
# 지원되는 모든 구성 매개변수 목록은 configuration.py에서 확인할 수 있습니다.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# API 클라이언트 인스턴스로 컨텍스트 진입
with client.ApiClient(configuration) as api_client:
    # API 클래스 인스턴스 생성
    api_instance = client.ModerationApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_id = 'comment_id_example' # str | 
    spam = True # bool |  (옵션)
    perm_not_spam = True # bool |  (옵션)
    broadcast_id = 'broadcast_id_example' # str |  (옵션)
    sso = 'sso_example' # str |  (옵션)

    try:
        api_response = api_instance.post_set_comment_spam_status(tenant_id, comment_id, PostSetCommentSpamStatusOptions(spam=spam, perm_not_spam=perm_not_spam, broadcast_id=broadcast_id, sso=sso))
        print("ModerationApi->post_set_comment_spam_status의 응답:\n")
        pprint(api_response)
    except Exception as e:
        print("ModerationApi->post_set_comment_spam_status 호출 중 예외 발생: %s\n" % e)
[inline-code-end]