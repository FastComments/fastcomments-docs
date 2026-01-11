req
tenantId
afterId

## 매개변수

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| afterId | string | query | 아니오 |  |
| limit | integer | query | 아니오 |  |
| tags | array | query | 아니오 |  |

## 응답

반환: [`GetFeedPosts200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_feed_posts200_response.py)

## 예제

[inline-code-attrs-start title = 'get_feed_posts 예제'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_feed_posts200_response import GetFeedPosts200Response
from client.rest import ApiException
from pprint import pprint

# 호스트 정의는 선택 사항이며 기본값은 https://fastcomments.com 입니다
# 지원되는 모든 구성 매개변수 목록은 configuration.py를 참조하세요.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# 클라이언트는 인증 및 권한 부여 매개변수를
# API 서버 보안 정책에 따라 구성해야 합니다.
# 아래에는 각 인증 방법에 대한 예제가 제공됩니다. 아래 예제 중
# 귀하의 인증 사용 사례에 맞는 예제를 사용하세요.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 필요하면 아래의 주석을 해제하여 API 키 접두사(예: Bearer)를 설정하세요
# configuration.api_key_prefix['api_key'] = 'Bearer'

# API 클라이언트 인스턴스를 사용하여 컨텍스트에 진입합니다
with client.ApiClient(configuration) as api_client:
    # API 클래스의 인스턴스를 생성합니다
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    after_id = 'after_id_example' # str |  (선택 사항)
    limit = 56 # int |  (선택 사항)
    tags = ['tags_example'] # List[str] |  (선택 사항)

    try:
        api_response = api_instance.get_feed_posts(tenant_id, after_id=after_id, limit=limit, tags=tags)
        print("The response of DefaultApi->get_feed_posts:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_feed_posts: %s\n" % e)
[inline-code-end]