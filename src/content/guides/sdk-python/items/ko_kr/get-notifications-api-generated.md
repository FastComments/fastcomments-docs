## 매개변수

| 이름 | 유형 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| userId | string | query | 아니오 |  |
| urlId | string | query | 아니오 |  |
| fromCommentId | string | query | 아니오 |  |
| viewed | boolean | query | 아니오 |  |
| type | string | query | 아니오 |  |
| skip | number | query | 아니오 |  |

## 응답

반환: [`GetNotificationsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_notifications_response.py)

## 예시

[inline-code-attrs-start title = 'get_notifications 예시'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.default_api import GetNotificationsOptions
from client.models.get_notifications_response import GetNotificationsResponse
from client.rest import ApiException
from pprint import pprint

# 호스트를 정의하는 것은 선택 사항이며 기본값은 https://fastcomments.com 입니다.
# 지원되는 모든 구성 매개변수 목록은 configuration.py 를 확인하십시오.
# 클라이언트는 API 서버 보안 정책에 따라 인증 및 권한 부여 매개변수를 구성해야 합니다.
# 아래에 각 인증 방식에 대한 예제가 제공되며, 사용 사례에 맞는 예제를 사용하십시오.
# API 키 인증 구성: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 필요에 따라 API 키에 접두사(예: Bearer)를 설정하려면 아래 주석을 해제하십시오
# configuration.api_key_prefix['api_key'] = 'Bearer'

# API 클라이언트 인스턴스로 컨텍스트에 들어갑니다
with client.ApiClient(configuration) as api_client:
    # API 클래스의 인스턴스를 생성합니다
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    user_id = 'user_id_example' # str |  (선택 사항)
    url_id = 'url_id_example' # str |  (선택 사항)
    from_comment_id = 'from_comment_id_example' # str |  (선택 사항)
    viewed = True # bool |  (선택 사항)
    type = 'type_example' # str |  (선택 사항)
    skip = 3.4 # float |  (선택 사항)

    try:
        api_response = api_instance.get_notifications(tenant_id, GetNotificationsOptions(user_id=user_id, url_id=url_id, from_comment_id=from_comment_id, viewed=viewed, type=type, skip=skip))
        print("The response of DefaultApi->get_notifications:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_notifications: %s\n" % e)
[inline-code-end]

---