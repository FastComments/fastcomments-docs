## 매개변수

| 이름 | 유형 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| text-search | string | query | 아니요 |  |
| byIPFromComment | string | query | 아니요 |  |
| filter | string | query | 아니요 |  |
| searchFilters | string | query | 아니요 |  |
| demo | boolean | query | 아니요 |  |
| sso | string | query | 아니요 |  |

## 응답

반환: [`ModerationAPICountCommentsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/moderation_api_count_comments_response.py)

## 예제

[inline-code-attrs-start title = 'get_count 예제'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.moderation_api_count_comments_response import ModerationAPICountCommentsResponse
from client.rest import ApiException
from pprint import pprint

# 호스트 정의는 선택 사항이며 기본값은 https://fastcomments.com
# 지원되는 모든 구성 매개변수 목록은 configuration.py를 참조하세요.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# API 클라이언트 인스턴스로 컨텍스트를 엽니다
with client.ApiClient(configuration) as api_client:
    # API 클래스의 인스턴스를 생성합니다
    api_instance = client.ModerationApi(api_client)
    text_search = 'text_search_example' # str |  (선택 사항)
    by_ip_from_comment = 'by_ip_from_comment_example' # str |  (선택 사항)
    filter = 'filter_example' # str |  (선택 사항)
    search_filters = 'search_filters_example' # str |  (선택 사항)
    demo = True # bool |  (선택 사항)
    sso = 'sso_example' # str |  (선택 사항)

    try:
        api_response = api_instance.get_count(text_search=text_search, by_ip_from_comment=by_ip_from_comment, filter=filter, search_filters=search_filters, demo=demo, sso=sso)
        print("The response of ModerationApi->get_count:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_count: %s\n" % e)
[inline-code-end]