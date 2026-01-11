## 매개변수

| 이름 | 타입 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| page | integer | query | 아니요 |  |
| limit | integer | query | 아니요 |  |
| skip | integer | query | 아니요 |  |
| asTree | boolean | query | 아니요 |  |
| skipChildren | integer | query | 아니요 |  |
| limitChildren | integer | query | 아니요 |  |
| maxTreeDepth | integer | query | 아니요 |  |
| urlId | string | query | 아니요 |  |
| userId | string | query | 아니요 |  |
| anonUserId | string | query | 아니요 |  |
| contextUserId | string | query | 아니요 |  |
| hashTag | string | query | 아니요 |  |
| parentId | string | query | 아니요 |  |
| direction | string | query | 아니요 |  |

## 응답

반환: [`GetComments200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_comments200_response.py)

## 예제

[inline-code-attrs-start title = 'get_comments 예제'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_comments200_response import GetComments200Response
from client.models.sort_directions import SortDirections
from client.rest import ApiException
from pprint import pprint

# 호스트 정의는 선택 사항이며 기본값은 https://fastcomments.com 입니다
# 지원되는 모든 구성 매개변수 목록은 configuration.py를 참조하세요.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# 클라이언트는 인증 및 권한 부여 매개변수를
# API 서버 보안 정책에 맞게 구성해야 합니다.
# 각 인증 방법에 대한 예제가 아래에 제공됩니다. 사용 사례에 맞는 예제를 사용하세요.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# configuration.api_key_prefix['api_key'] = 'Bearer'

# API 클라이언트 인스턴스로 컨텍스트에 들어갑니다
with client.ApiClient(configuration) as api_client:
    # API 클래스의 인스턴스를 생성합니다
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    page = 56 # int |  (선택 사항)
    limit = 56 # int |  (선택 사항)
    skip = 56 # int |  (선택 사항)
    as_tree = True # bool |  (선택 사항)
    skip_children = 56 # int |  (선택 사항)
    limit_children = 56 # int |  (선택 사항)
    max_tree_depth = 56 # int |  (선택 사항)
    url_id = 'url_id_example' # str |  (선택 사항)
    user_id = 'user_id_example' # str |  (선택 사항)
    anon_user_id = 'anon_user_id_example' # str |  (선택 사항)
    context_user_id = 'context_user_id_example' # str |  (선택 사항)
    hash_tag = 'hash_tag_example' # str |  (선택 사항)
    parent_id = 'parent_id_example' # str |  (선택 사항)
    direction = client.SortDirections() # SortDirections |  (선택 사항)

    try:
        api_response = api_instance.get_comments(tenant_id, page=page, limit=limit, skip=skip, as_tree=as_tree, skip_children=skip_children, limit_children=limit_children, max_tree_depth=max_tree_depth, url_id=url_id, user_id=user_id, anon_user_id=anon_user_id, context_user_id=context_user_id, hash_tag=hash_tag, parent_id=parent_id, direction=direction)
        print("The response of DefaultApi->get_comments:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_comments: %s\n" % e)
[inline-code-end]