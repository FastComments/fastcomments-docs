---
## 매개변수

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 아니요 |  |

## 응답

반환: [`AddHashTagsBulk200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/add_hash_tags_bulk200_response.py)

## 예제

[inline-code-attrs-start title = 'add_hash_tags_bulk 예제'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.add_hash_tags_bulk200_response import AddHashTagsBulk200Response
from client.models.bulk_create_hash_tags_body import BulkCreateHashTagsBody
from client.rest import ApiException
from pprint import pprint

# 호스트 정의는 선택 사항이며 기본값은 https://fastcomments.com 입니다
# 모든 지원되는 구성 매개변수 목록은 configuration.py를 참조하세요.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# 클라이언트는 인증 및 권한 부여 매개변수를 구성해야 합니다
# API 서버 보안 정책에 따라.
# 각 인증 방법에 대한 예제가 아래에 제공되며, 자신의 인증 사용 사례에
# 맞는 예제를 사용하세요.

# API 키 인증 구성: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 필요하면 API 키에 대한 접두사(예: Bearer)를 설정하려면 아래의 주석을 제거하세요
# configuration.api_key_prefix['api_key'] = 'Bearer'

# API 클라이언트 인스턴스와 함께 컨텍스트에 진입하세요
with client.ApiClient(configuration) as api_client:
    # API 클래스의 인스턴스를 생성하세요
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str |  (선택 사항)
    bulk_create_hash_tags_body = client.BulkCreateHashTagsBody() # BulkCreateHashTagsBody |  (선택 사항)

    try:
        api_response = api_instance.add_hash_tags_bulk(tenant_id=tenant_id, bulk_create_hash_tags_body=bulk_create_hash_tags_body)
        print("The response of DefaultApi->add_hash_tags_bulk:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->add_hash_tags_bulk: %s\n" % e)
[inline-code-end]

---