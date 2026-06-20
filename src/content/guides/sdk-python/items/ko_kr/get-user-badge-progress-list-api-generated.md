## 매개변수

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| userId | string | query | 아니오 |  |
| limit | number | query | 아니오 |  |
| skip | number | query | 아니오 |  |

## 응답

반환: [`APIGetUserBadgeProgressListResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_get_user_badge_progress_list_response.py)

## 예제

[inline-code-attrs-start title = 'get_user_badge_progress_list 예제'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_get_user_badge_progress_list_response import APIGetUserBadgeProgressListResponse
from client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to https://fastcomments.com
# configuration.py에서 지원되는 모든 구성 매개변수 목록을 참조하세요.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# The client must configure the authentication and authorization parameters
# 인증 및 권한 부여 매개변수를 API 서버 보안 정책에 따라 구성해야 합니다.
# Examples for each auth method are provided below, use the example that
# 각 인증 방법에 대한 예제가 아래에 제공됩니다. 다음 예제 중
# 귀하의 인증 사용 사례에 맞는 것을 사용하세요.
# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# 필요하면 아래의 주석을 해제하여 API 키에 접두사(예: Bearer)를 설정하세요
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Enter a context with an instance of the API client
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    # API 클래스의 인스턴스를 생성합니다
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    user_id = 'user_id_example' # str |  (선택 사항)
    limit = 3.4 # float |  (선택 사항)
    skip = 3.4 # float |  (선택 사항)

    try:
        api_response = api_instance.get_user_badge_progress_list(tenant_id, user_id=user_id, limit=limit, skip=skip)
        print("The response of DefaultApi->get_user_badge_progress_list:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_user_badge_progress_list: %s\n" % e)
[inline-code-end]