## 매개변수

| 이름 | 형식 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 예 |  |
| commentId | string | path | 예 |  |
| broadcastId | string | query | 예 |  |
| sso | string | query | 아니오 |  |

## 응답

반환: [`PinComment200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/pin_comment200_response.py)

## 예제

[inline-code-attrs-start title = 'pin_comment 예제'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.pin_comment200_response import PinComment200Response
from client.rest import ApiException
from pprint import pprint

# 호스트 정의는 선택 사항이며 기본값은 https://fastcomments.com 입니다
# 지원되는 모든 구성 매개변수 목록은 configuration.py를 참조하세요.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# API 클라이언트 인스턴스를 사용하여 컨텍스트에 진입합니다
with client.ApiClient(configuration) as api_client:
    # API 클래스 인스턴스를 생성합니다
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_id = 'comment_id_example' # str | 
    broadcast_id = 'broadcast_id_example' # str | 
    sso = 'sso_example' # str |  (선택 사항)

    try:
        api_response = api_instance.pin_comment(tenant_id, comment_id, broadcast_id, sso=sso)
        print("The response of PublicApi->pin_comment:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->pin_comment: %s\n" % e)
[inline-code-end]