## 매개변수

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| id | string | path | 예 |  |
| updateComments | string | query | 아니요 |  |

## 응답

반환: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_empty_response.py)

## 예제

[inline-code-attrs-start title = 'replace_tenant_user 예제'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_empty_response import APIEmptyResponse
from client.models.replace_tenant_user_body import ReplaceTenantUserBody
from client.rest import ApiException
from pprint import pprint

# 호스트 정의는 선택 사항이며 기본값은 https://fastcomments.com 입니다
# 지원되는 모든 구성 매개변수 목록은 configuration.py를 참조하세요.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# 클라이언트는 인증 및 권한 부여 매개변수를 구성해야 합니다
# 이는 API 서버의 보안 정책에 따라야 합니다
# 각 인증 방법에 대한 예제가 아래에 제공됩니다. 사용 사례에
# 맞는 예제를 사용하세요.

# API 키 인증 구성: api_key

# 필요하다면 아래 주석을 제거하여 API 키의 접두사(예: Bearer)를 설정하세요

# API 클라이언트 인스턴스를 사용하여 컨텍스트에 진입합니다
with client.ApiClient(configuration) as api_client:
    # API 클래스의 인스턴스를 생성합니다
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    replace_tenant_user_body = client.ReplaceTenantUserBody() # ReplaceTenantUserBody | 
    update_comments = 'update_comments_example' # str |  (optional)

    try:
        api_response = api_instance.replace_tenant_user(tenant_id, id, replace_tenant_user_body, update_comments=update_comments)
        print("The response of DefaultApi->replace_tenant_user:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->replace_tenant_user: %s\n" % e)
[inline-code-end]