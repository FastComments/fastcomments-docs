## 매개변수

| 이름 | 유형 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |
| contextUserId | string | query | No |  |
| doSpamCheck | boolean | query | No |  |
| isLive | boolean | query | No |  |

## 응답

Returns: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_empty_response.py)

## 예제

[inline-code-attrs-start title = 'update_comment 예제'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.default_api import UpdateCommentOptions
from client.models.api_empty_response import APIEmptyResponse
from client.models.updatable_comment_params import UpdatableCommentParams
from client.rest import ApiException
from pprint import pprint

# 호스트 정의는 선택 사항이며 기본값은 https://fastcomments.com 입니다
# 지원되는 모든 구성 매개변수 목록은 configuration.py를 참조하세요.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# 클라이언트는 API 서버 보안 정책에 따라 인증 및 인가 매개변수를 구성해야 합니다.
# 아래에 각 인증 방법에 대한 예제가 제공되며, 사용 사례에 맞는 예제를 사용하세요.
# API 키 인증 설정: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 필요 시 API 키에 대한 접두사(e.g. Bearer)를 설정하려면 아래 주석을 해제하세요
# configuration.api_key_prefix['api_key'] = 'Bearer'

# API 클라이언트 인스턴스로 컨텍스트에 진입합니다
with client.ApiClient(configuration) as api_client:
    # API 클래스의 인스턴스를 생성합니다
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    updatable_comment_params = client.UpdatableCommentParams() # UpdatableCommentParams | 
    context_user_id = 'context_user_id_example' # str |  (옵션)
    do_spam_check = True # bool |  (옵션)
    is_live = True # bool |  (옵션)

    try:
        api_response = api_instance.update_comment(tenant_id, id, updatable_comment_params, UpdateCommentOptions(context_user_id=context_user_id, do_spam_check=do_spam_check, is_live=is_live))
        print("The response of DefaultApi->update_comment:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->update_comment: %s\n" % e)
[inline-code-end]