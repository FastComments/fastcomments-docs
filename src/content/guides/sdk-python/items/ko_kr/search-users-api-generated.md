## 매개변수

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | 예 |  |
| urlId | string | query | 예 |  |
| usernameStartsWith | string | query | 아니요 |  |
| mentionGroupIds | array | query | 아니요 |  |
| sso | string | query | 아니요 |  |
| searchSection | string | query | 아니요 |  |

## 응답

반환: [`SearchUsers200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/search_users200_response.py)

## 예제

[inline-code-attrs-start title = 'search_users 예제'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.search_users200_response import SearchUsers200Response
from client.rest import ApiException
from pprint import pprint

# 호스트 정의는 선택 사항이며 기본값은 https://fastcomments.com 입니다
# 사용 가능한 모든 구성 매개변수 목록은 configuration.py를 참조하세요.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# API 클라이언트 인스턴스로 컨텍스트를 엽니다
with client.ApiClient(configuration) as api_client:
    # API 클래스의 인스턴스를 생성합니다
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | 
    username_starts_with = 'username_starts_with_example' # str |  (선택 사항)
    mention_group_ids = ['mention_group_ids_example'] # List[str] |  (선택 사항)
    sso = 'sso_example' # str |  (선택 사항)
    search_section = 'search_section_example' # str |  (선택 사항)

    try:
        api_response = api_instance.search_users(tenant_id, url_id, username_starts_with=username_starts_with, mention_group_ids=mention_group_ids, sso=sso, search_section=search_section)
        print("The response of PublicApi->search_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->search_users: %s\n" % e)
[inline-code-end]