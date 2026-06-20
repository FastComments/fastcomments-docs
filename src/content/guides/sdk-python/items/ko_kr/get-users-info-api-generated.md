---
테넌트의 대량 사용자 정보입니다. 주어진 userIds에 대해 User / SSOUser의 표시 정보를 반환합니다.
댓글 위젯에서 프레즌스 이벤트로 방금 나타난 사용자 정보를 보강하는 데 사용됩니다.
페이지 컨텍스트 없음: 개인정보 보호가 일관되게 적용됩니다(비공개 프로필은 마스킹됨).

## 매개변수

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| ids | string | query | Yes | 쉼표로 구분된 userIds. |

## 응답

반환: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/page_users_info_response.py)

## 예제

[inline-code-attrs-start title = 'get_users_info 예제'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.page_users_info_response import PageUsersInfoResponse
from client.rest import ApiException
from pprint import pprint

# 호스트 설정은 선택 사항이며 기본값은 https://fastcomments.com 입니다
# 모든 지원되는 구성 매개변수 목록은 configuration.py를 참조하세요.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# API 클라이언트 인스턴스와 함께 컨텍스트에 들어갑니다
with client.ApiClient(configuration) as api_client:
    # API 클래스의 인스턴스를 생성합니다
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    ids = 'ids_example' # str | 쉼표로 구분된 userIds.

    try:
        api_response = api_instance.get_users_info(tenant_id, ids)
        print("The response of PublicApi->get_users_info:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_users_info: %s\n" % e)
[inline-code-end]

---