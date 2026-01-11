## 매개변수

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| id | string | path | 예 |  |
| userId | string | query | 아니오 |  |
| anonUserId | string | query | 아니오 |  |

## 응답

반환: [`FlagComment200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/flag_comment200_response.py)

## 예제

[inline-code-attrs-start title = 'flag_comment 예제'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.flag_comment200_response import FlagComment200Response
from client.rest import ApiException
from pprint import pprint

# 호스트 정의는 선택사항이며 기본값은 https://fastcomments.com 입니다
# 지원되는 모든 구성 매개변수 목록은 configuration.py를 참조하세요.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# 클라이언트는 API 서버 보안 정책에 따라 인증 및 권한 부여 매개변수를 구성해야 합니다.
# 아래에 각 인증 방법에 대한 예제가 제공됩니다. 사용 사례에 맞는 예제를 사용하세요.
# API 키 인증 구성: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 필요한 경우 API 키에 대한 접두사(예: Bearer)를 설정하려면 아래의 주석을 해제하세요
# configuration.api_key_prefix['api_key'] = 'Bearer'

# API 클라이언트 인스턴스와 함께 컨텍스트를 엽니다
with client.ApiClient(configuration) as api_client:
    # API 클래스의 인스턴스를 생성합니다
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    user_id = 'user_id_example' # str |  (선택 사항)
    anon_user_id = 'anon_user_id_example' # str |  (선택 사항)

    try:
        api_response = api_instance.flag_comment(tenant_id, id, user_id=user_id, anon_user_id=anon_user_id)
        print("The response of DefaultApi->flag_comment:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->flag_comment: %s\n" % e)
[inline-code-end]