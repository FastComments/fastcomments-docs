## 매개변수

| 이름 | 형식 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| userId | string | query | 아니요 |  |
| urlId | string | query | 아니요 |  |
| fromCommentId | string | query | 아니요 |  |
| viewed | boolean | query | 아니요 |  |
| type | string | query | 아니요 |  |
| skip | number | query | 아니요 |  |

## 응답

반환: [`GetNotifications200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_notifications200_response.py)

## 예제

[inline-code-attrs-start title = 'get_notifications 예제'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_notifications200_response import GetNotifications200Response
from client.rest import ApiException
from pprint import pprint

# 호스트를 정의하는 것은 선택 사항이며 기본값은 https://fastcomments.com 입니다.
# 지원되는 모든 구성 매개변수 목록은 configuration.py를 참조하세요.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# 클라이언트는 API 서버 보안 정책에 따라 인증 및 권한 부여 매개변수를 구성해야 합니다.
# 각 인증 방법에 대한 예제가 아래에 제공됩니다. 귀하의 인증 사용 사례에 맞는 예제를 사용하세요.

# API 키 인증 구성: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 필요하면 아래의 주석 처리를 해제하여 API 키 접두사(예: Bearer)를 설정하세요
# configuration.api_key_prefix['api_key'] = 'Bearer'

# API 클라이언트 인스턴스를 사용하여 컨텍스트를 엽니다
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
        api_response = api_instance.get_notifications(tenant_id, user_id=user_id, url_id=url_id, from_comment_id=from_comment_id, viewed=viewed, type=type, skip=skip)
        print("The response of DefaultApi->get_notifications:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_notifications: %s\n" % e)
[inline-code-end]