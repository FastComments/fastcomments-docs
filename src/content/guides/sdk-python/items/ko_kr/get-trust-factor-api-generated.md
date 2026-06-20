## 매개변수

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| userId | string | query | 아니요 |  |
| sso | string | query | 아니요 |  |

## 응답

반환: [`GetUserTrustFactorResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_user_trust_factor_response.py)

## 예제

[inline-code-attrs-start title = 'get_trust_factor 예제'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_user_trust_factor_response import GetUserTrustFactorResponse
from client.rest import ApiException
from pprint import pprint

# 호스트를 지정하는 것은 선택 사항이며 기본값은 https://fastcomments.com 입니다.
# 지원되는 모든 구성 매개변수 목록은 configuration.py를 참조하세요.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# API 클라이언트 인스턴스로 컨텍스트를 엽니다.
with client.ApiClient(configuration) as api_client:
    # API 클래스의 인스턴스를 생성합니다.
    api_instance = client.ModerationApi(api_client)
    user_id = 'user_id_example' # str |  (선택 사항)
    sso = 'sso_example' # str |  (선택 사항)

    try:
        api_response = api_instance.get_trust_factor(user_id=user_id, sso=sso)
        print("The response of ModerationApi->get_trust_factor:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_trust_factor: %s\n" % e)
[inline-code-end]