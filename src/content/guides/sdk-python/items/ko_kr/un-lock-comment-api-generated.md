## 매개변수

| 이름 | 형식 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 예 |  |
| commentId | string | path | 예 |  |
| broadcastId | string | query | 예 |  |
| sso | string | query | 아니요 |  |

## 응답

반환: [`LockComment200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/lock_comment200_response.py)

## 예제

[inline-code-attrs-start title = 'un_lock_comment 예제'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.lock_comment200_response import LockComment200Response
from client.rest import ApiException
from pprint import pprint

# 호스트를 정의하는 것은 선택 사항이며 기본값은 https://fastcomments.com 입니다
# 모든 지원 구성 매개변수 목록은 configuration.py를 참조하세요.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# API 클라이언트 인스턴스로 컨텍스트를 엽니다
with client.ApiClient(configuration) as api_client:
    # API 클래스의 인스턴스를 생성합니다
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_id = 'comment_id_example' # str | 
    broadcast_id = 'broadcast_id_example' # str | 
    sso = 'sso_example' # str |  (선택 사항)

    try:
        api_response = api_instance.un_lock_comment(tenant_id, comment_id, broadcast_id, sso=sso)
        print("The response of PublicApi->un_lock_comment:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->un_lock_comment: %s\n" % e)
[inline-code-end]

---