## 매개변수

| 이름 | 유형 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | 쿼리 | 예 |  |
| id | string | 경로 | 예 |  |

## 응답

반환: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/flag_comment_public200_response.py)

## 예제

[inline-code-attrs-start title = 'delete_question_config 예제'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.flag_comment_public200_response import FlagCommentPublic200Response
from client.rest import ApiException
from pprint import pprint

# 호스트 정의는 선택사항이며 기본값은 https://fastcomments.com 입니다
# 모든 지원되는 구성 매개변수의 목록은 configuration.py를 참조하세요.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# 클라이언트는 인증(authentication) 및 권한 부여(authorization) 매개변수를 설정해야 합니다
# API 서버 보안 정책에 따라.
# 각 인증 방법에 대한 예제가 아래에 제공됩니다. 아래 예제 중에서
# 인증 사용 사례에 맞는 예제를 사용하세요.
# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 필요하면 API 키에 대한 접두사(예: Bearer)를 설정하려면 아래의 주석을 해제하세요
# configuration.api_key_prefix['api_key'] = 'Bearer'

# API 클라이언트 인스턴스를 사용하여 컨텍스트를 엽니다
with client.ApiClient(configuration) as api_client:
    # API 클래스의 인스턴스를 생성합니다
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 

    try:
        api_response = api_instance.delete_question_config(tenant_id, id)
        print("The response of DefaultApi->delete_question_config:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->delete_question_config: %s\n" % e)
[inline-code-end]

---