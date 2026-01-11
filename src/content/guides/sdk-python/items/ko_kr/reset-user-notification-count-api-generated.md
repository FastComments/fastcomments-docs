---
## 매개변수

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| sso | string | query | 아니오 |  |

## 응답

반환: [`ResetUserNotifications200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/reset_user_notifications200_response.py)

## 예제

[inline-code-attrs-start title = 'reset_user_notification_count 예제'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.reset_user_notifications200_response import ResetUserNotifications200Response
from client.rest import ApiException
from pprint import pprint

# 호스트를 정의하는 것은 선택사항이며 기본값은 https://fastcomments.com 입니다
# 지원되는 모든 구성 매개변수 목록은 configuration.py를 참조하세요.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# API 클라이언트 인스턴스와 함께 컨텍스트를 엽니다
with client.ApiClient(configuration) as api_client:
    # API 클래스의 인스턴스를 생성합니다
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    sso = 'sso_example' # str |  (선택사항)

    try:
        api_response = api_instance.reset_user_notification_count(tenant_id, sso=sso)
        print("The response of PublicApi->reset_user_notification_count:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->reset_user_notification_count: %s\n" % e)
[inline-code-end]

---