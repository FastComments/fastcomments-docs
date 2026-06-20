请求
tenantId
afterId

## 参数

| 名称 | 类型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | 查询 | 是 |  |
| afterId | string | 查询 | 否 |  |
| limit | integer | 查询 | 否 |  |
| tags | array | 查询 | 否 |  |

## 响应

返回: [`GetFeedPostsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_feed_posts_response.py)

## 示例

[inline-code-attrs-start title = 'get_feed_posts 示例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_feed_posts_response import GetFeedPostsResponse
from client.rest import ApiException
from pprint import pprint

# 定义 host 是可选的，默认值为 https://fastcomments.com
# 请参阅 configuration.py 获取所有支持的配置参数列表。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# 客户端必须根据 API 服务器的安全策略配置身份验证和授权参数。
# 下面提供了每种认证方法的示例，请使用满足您认证用例的示例。

# 配置 API 密钥授权: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 如有需要，请取消注释以下行以为 API 密钥设置前缀（例如 Bearer）
# configuration.api_key_prefix['api_key'] = 'Bearer'

# 在一个上下文中使用 API 客户端实例
with client.ApiClient(configuration) as api_client:
    # 创建 API 类的实例
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    after_id = 'after_id_example' # str |  (optional)
    limit = 56 # int |  (optional)
    tags = ['tags_example'] # List[str] |  (optional)

    try:
        api_response = api_instance.get_feed_posts(tenant_id, after_id=after_id, limit=limit, tags=tags)
        print("The response of DefaultApi->get_feed_posts:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_feed_posts: %s\n" % e)
[inline-code-end]