## 매개변수

| 이름 | 유형 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| commentId | string | path | 예 |  |
| sso | string | query | 아니오 |  |

## 응답

반환: [`GetCommentTextResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_comment_text_response.py)

## 예제

[inline-code-attrs-start title = 'get_moderation_comment_text 예제'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_comment_text_response import GetCommentTextResponse
from client.rest import ApiException
from pprint import pprint

# 호스트 정의는 선택 사항이며 기본값은 https://fastcomments.com 입니다
# 모든 지원되는 구성 매개변수 목록은 configuration.py를 참조하세요.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# API 클라이언트 인스턴스로 컨텍스트에 진입합니다
with client.ApiClient(configuration) as api_client:
    # API 클래스의 인스턴스를 생성합니다
    api_instance = client.ModerationApi(api_client)
    comment_id = 'comment_id_example' # str | 
    sso = 'sso_example' # str |  (선택 사항)

    try:
        api_response = api_instance.get_moderation_comment_text(comment_id, sso=sso)
        print("The response of ModerationApi->get_moderation_comment_text:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_moderation_comment_text: %s\n" % e)
[inline-code-end]