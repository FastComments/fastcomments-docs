## 매개변수

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | path | 예 |  |
| includeByUserIdAndEmail | boolean | query | 아니오 |  |
| includeByIP | boolean | query | 아니오 |  |
| includeByEmailDomain | boolean | query | 아니오 |  |
| sso | string | query | 아니오 |  |

## 응답

반환: [`PreBanSummary`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/pre_ban_summary.py)

## 예제

[inline-code-attrs-start title = 'get_pre_ban_summary 예제'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.pre_ban_summary import PreBanSummary
from client.rest import ApiException
from pprint import pprint

# 호스트 정의는 선택 사항이며 기본값은 https://fastcomments.com 입니다
# 모든 지원 구성 매개변수의 목록은 configuration.py를 참조하세요.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# API 클라이언트 인스턴스로 컨텍스트를 엽니다
with client.ApiClient(configuration) as api_client:
    # API 클래스의 인스턴스를 생성합니다
    api_instance = client.ModerationApi(api_client)
    comment_id = 'comment_id_example' # str | 
    include_by_user_id_and_email = True # bool |  (선택 사항)
    include_by_ip = True # bool |  (선택 사항)
    include_by_email_domain = True # bool |  (선택 사항)
    sso = 'sso_example' # str |  (선택 사항)

    try:
        api_response = api_instance.get_pre_ban_summary(comment_id, include_by_user_id_and_email=include_by_user_id_and_email, include_by_ip=include_by_ip, include_by_email_domain=include_by_email_domain, sso=sso)
        print("The response of ModerationApi->get_pre_ban_summary:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_pre_ban_summary: %s\n" % e)
[inline-code-end]