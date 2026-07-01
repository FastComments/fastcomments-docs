## 매개변수

| 이름 | 유형 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| tag | string | path | 예 |  |

## 응답

Returns: [`UpdateHashTagResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/update_hash_tag_response.py)

## 예시

[inline-code-attrs-start title = 'patch_hash_tag 예제'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.update_hash_tag_body import UpdateHashTagBody
from client.models.update_hash_tag_response import UpdateHashTagResponse
from client.rest import ApiException
from pprint import pprint

# 호스트 정의는 선택 사항이며 기본값은 https://fastcomments.com 입니다
# configuration.py에서 지원되는 모든 구성 매개변수 목록을 확인하세요.
# 클라이언트는 인증 및 인가 매개변수를 구성해야 합니다
# API 서버 보안 정책에 따라
# 각 인증 방법에 대한 예제가 아래에 제공됩니다. 해당 예제를 사용하여
# 귀하의 인증 사용 사례에 맞는 예제를 선택하세요.

# API 키 인증 구성: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 필요에 따라 API 키 접두사(예: Bearer)를 설정하려면 아래 주석을 해제하세요
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Enter a context with an instance of the API client
with client.ApiClient(configuration) as api_client:
    # API 클래스의 인스턴스를 생성합니다
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    tag = 'tag_example' # str | 
    update_hash_tag_body = client.UpdateHashTagBody() # UpdateHashTagBody |  (optional)

    try:
        api_response = api_instance.patch_hash_tag(tenant_id, tag, update_hash_tag_body)
        print("The response of DefaultApi->patch_hash_tag:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->patch_hash_tag: %s\n" % e)
[inline-code-end]