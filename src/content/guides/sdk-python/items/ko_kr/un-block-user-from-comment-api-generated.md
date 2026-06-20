## 매개변수

| 이름 | 타입 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| id | string | path | 예 |  |
| userId | string | query | 아니요 |  |
| anonUserId | string | query | 아니요 |  |

## 응답

반환: [`UnblockSuccess`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/unblock_success.py)

## 예제

[inline-code-attrs-start title = 'un_block_user_from_comment 예제'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.un_block_from_comment_params import UnBlockFromCommentParams
from client.models.unblock_success import UnblockSuccess
from client.rest import ApiException
from pprint import pprint

# 호스트 정의는 선택 사항이며 기본값은 https://fastcomments.com 입니다
# 지원되는 모든 구성 매개변수 목록은 configuration.py를 참조하세요.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# 클라이언트는 인증 및 권한 부여 매개변수를
# API 서버 보안 정책에 따라 구성해야 합니다.
# 아래에는 각 인증 방법에 대한 예제가 제공되며, 사용 사례에
# 맞는 예제를 사용하세요.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 필요하면 API 키에 대한 접두사(예: Bearer)를 설정하려면 아래 주석을 해제하세요
# configuration.api_key_prefix['api_key'] = 'Bearer'

# API 클라이언트 인스턴스로 컨텍스트에 진입합니다
with client.ApiClient(configuration) as api_client:
    # API 클래스의 인스턴스를 생성합니다
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    un_block_from_comment_params = client.UnBlockFromCommentParams() # UnBlockFromCommentParams | 
    user_id = 'user_id_example' # str |  (선택 사항)
    anon_user_id = 'anon_user_id_example' # str |  (선택 사항)

    try:
        api_response = api_instance.un_block_user_from_comment(tenant_id, id, un_block_from_comment_params, user_id=user_id, anon_user_id=anon_user_id)
        print("The response of DefaultApi->un_block_user_from_comment:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->un_block_user_from_comment: %s\n" % e)
[inline-code-end]