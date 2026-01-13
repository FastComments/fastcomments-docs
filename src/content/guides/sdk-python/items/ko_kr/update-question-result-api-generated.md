## Parameters

| 이름 | 유형 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| id | string | path | 예 |  |

## 응답

반환: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/flag_comment_public200_response.py)

## 예제

[inline-code-attrs-start title = 'update_question_result 예제'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.flag_comment_public200_response import FlagCommentPublic200Response
from client.models.update_question_result_body import UpdateQuestionResultBody
from client.rest import ApiException
from pprint import pprint

# 호스트 정의는 선택 사항이며 기본값은 https://fastcomments.com 입니다
# 지원되는 모든 구성 매개변수 목록은 configuration.py를 참조하세요.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# 클라이언트는 API 서버 보안 정책에 따라
# 인증 및 권한 부여 매개변수를 구성해야 합니다.
# 아래에 각 인증 방법에 대한 예제가 제공됩니다. 귀하의 인증 사용 사례에
# 맞는 예제를 사용하세요.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 필요한 경우 API 키에 대한 접두사(예: Bearer)를 설정하려면 아래의 주석을 해제하세요
# configuration.api_key_prefix['api_key'] = 'Bearer'

# API 클라이언트 인스턴스와 함께 컨텍스트에 들어갑니다
with client.ApiClient(configuration) as api_client:
    # API 클래스 인스턴스 생성
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    update_question_result_body = client.UpdateQuestionResultBody() # UpdateQuestionResultBody | 

    try:
        api_response = api_instance.update_question_result(tenant_id, id, update_question_result_body)
        print("The response of DefaultApi->update_question_result:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->update_question_result: %s\n" % e)
[inline-code-end]