## 매개변수

| 이름 | 유형 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tag | string | 경로 | 예 |  |
| tenantId | string | 쿼리 | 아니오 |  |

## 응답

반환: [`PatchHashTag200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/patch_hash_tag200_response.py)

## 예제

[inline-code-attrs-start title = 'patch_hash_tag 예제'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.patch_hash_tag200_response import PatchHashTag200Response
from client.models.update_hash_tag_body import UpdateHashTagBody
from client.rest import ApiException
from pprint import pprint

# 호스트 정의는 선택 사항이며 기본값은 https://fastcomments.com 입니다
# 모든 지원되는 구성 매개변수 목록은 configuration.py를 참조하세요.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# 클라이언트는 API 서버 보안 정책에 따라 인증 및 권한 부여 매개변수를 구성해야 합니다.
# 아래에는 각 인증 방법의 예가 제공됩니다. 사용 사례에 맞는 예제를 사용하세요.

# API 키 인증 구성: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 필요하면 아래 주석을 해제하여 API 키의 접두사(e.g. Bearer)를 설정하세요
# configuration.api_key_prefix['api_key'] = 'Bearer'

# API 클라이언트 인스턴스와 함께 컨텍스트에 진입합니다
with client.ApiClient(configuration) as api_client:
    # API 클래스의 인스턴스를 생성합니다
    api_instance = client.DefaultApi(api_client)
    tag = 'tag_example' # str | 
    tenant_id = 'tenant_id_example' # str |  (선택 사항)
    update_hash_tag_body = client.UpdateHashTagBody() # UpdateHashTagBody |  (선택 사항)

    try:
        api_response = api_instance.patch_hash_tag(tag, tenant_id=tenant_id, update_hash_tag_body=update_hash_tag_body)
        print("The response of DefaultApi->patch_hash_tag:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->patch_hash_tag: %s\n" % e)
[inline-code-end]