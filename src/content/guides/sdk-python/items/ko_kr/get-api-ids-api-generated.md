## 매개변수

| 이름 | 유형 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| text-search | string | query | No |  |
| byIPFromComment | string | query | No |  |
| filters | string | query | No |  |
| searchFilters | string | query | No |  |
| afterId | string | query | No |  |
| demo | boolean | query | No |  |
| sso | string | query | No |  |

## 응답

반환: [`ModerationAPIGetCommentIdsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/moderation_api_get_comment_ids_response.py)

## 예시

[inline-code-attrs-start title = 'get_api_ids 예시'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.moderation_api import GetApiIdsOptions
from client.models.moderation_api_get_comment_ids_response import ModerationAPIGetCommentIdsResponse
from client.rest import ApiException
from pprint import pprint

# 호스트 정의는 선택 사항이며 기본값은 https://fastcomments.com 입니다.
# configuration.py에서 지원되는 모든 구성 매개변수 목록을 확인하십시오.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# API 클라이언트 인스턴스로 컨텍스트에 들어갑니다
with client.ApiClient(configuration) as api_client:
    # API 클래스의 인스턴스를 생성합니다
    api_instance = client.ModerationApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    text_search = 'text_search_example' # str |  (옵션)
    by_ip_from_comment = 'by_ip_from_comment_example' # str |  (옵션)
    filters = 'filters_example' # str |  (옵션)
    search_filters = 'search_filters_example' # str |  (옵션)
    after_id = 'after_id_example' # str |  (옵션)
    demo = True # bool |  (옵션)
    sso = 'sso_example' # str |  (옵션)

    try:
        api_response = api_instance.get_api_ids(tenant_id, GetApiIdsOptions(text_search=text_search, by_ip_from_comment=by_ip_from_comment, filters=filters, search_filters=search_filters, after_id=after_id, demo=demo, sso=sso))
        print("The response of ModerationApi->get_api_ids:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_api_ids: %s\n" % e)
[inline-code-end]