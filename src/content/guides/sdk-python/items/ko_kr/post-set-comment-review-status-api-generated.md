## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | path | Yes |  |
| reviewed | boolean | query | No |  |
| sso | string | query | No |  |

## Response

Returns: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_empty_response.py)

## Example

[inline-code-attrs-start title = 'post_set_comment_review_status 예제'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_empty_response import APIEmptyResponse
from client.rest import ApiException
from pprint import pprint

# 호스트 정의는 선택 사항이며 기본값은 https://fastcomments.com 입니다
# 지원되는 모든 구성 매개변수 목록은 configuration.py를 참조하세요.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# API 클라이언트 인스턴스와 함께 컨텍스트를 엽니다
with client.ApiClient(configuration) as api_client:
    # API 클래스 인스턴스를 생성합니다
    api_instance = client.ModerationApi(api_client)
    comment_id = 'comment_id_example' # str | 
    reviewed = True # bool |  (선택 사항)
    sso = 'sso_example' # str |  (선택 사항)

    try:
        api_response = api_instance.post_set_comment_review_status(comment_id, reviewed=reviewed, sso=sso)
        print("The response of ModerationApi->post_set_comment_review_status:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->post_set_comment_review_status: %s\n" % e)
[inline-code-end]