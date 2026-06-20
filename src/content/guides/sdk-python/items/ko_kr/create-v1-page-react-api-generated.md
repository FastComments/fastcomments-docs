## 매개변수

| 이름 | 유형 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 예 |  |
| urlId | string | query | 예 |  |
| title | string | query | 아니오 |  |

## 응답

반환: [`CreateV1PageReact`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/create_v1_page_react.py)

## 예제

[inline-code-attrs-start title = 'create_v1_page_react 예제'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.create_v1_page_react import CreateV1PageReact
from client.rest import ApiException
from pprint import pprint

# 호스트 정의는 선택 사항이며 기본값은 https://fastcomments.com 입니다
# 지원되는 모든 구성 매개변수 목록은 configuration.py를 참조하세요.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# API 클라이언트 인스턴스로 컨텍스트에 진입합니다
with client.ApiClient(configuration) as api_client:
    # API 클래스의 인스턴스를 생성합니다
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | 
    title = 'title_example' # str |  (optional)

    try:
        api_response = api_instance.create_v1_page_react(tenant_id, url_id, title=title)
        print("The response of PublicApi->create_v1_page_react:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->create_v1_page_react: %s\n" % e)
[inline-code-end]