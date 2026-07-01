## 参数

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| broadcastId | string | query | No |  |
| isLive | boolean | query | No |  |
| doSpamCheck | boolean | query | No |  |
| skipDupCheck | boolean | query | No |  |

## 响应

返回: [`CreateFeedPostsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/create_feed_posts_response.py)

## 示例

[inline-code-attrs-start title = 'create_feed_post 示例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.default_api import CreateFeedPostOptions
from client.models.create_feed_post_params import CreateFeedPostParams
from client.models.create_feed_posts_response import CreateFeedPostsResponse
from client.rest import ApiException
from pprint import pprint

# 定义 host 是可选的，默认为 https://fastcomments.com
# 查看 configuration.py 获取所有支持的配置参数列表。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# 客户端必须根据 API 服务器的安全策略配置身份验证和授权参数。
# 为每种身份验证方法提供了示例，请使用符合您身份验证使用情况的示例。

# 配置 API 密钥授权: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 如有需要，取消注释以下行以设置 API 密钥的前缀（例如 Bearer）
# configuration.api_key_prefix['api_key'] = 'Bearer'

# 进入带有 API 客户端实例的上下文
with client.ApiClient(configuration) as api_client:
    # 创建 API 类的实例
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    create_feed_post_params = client.CreateFeedPostParams() # CreateFeedPostParams | 
    broadcast_id = 'broadcast_id_example' # str |  （可选）
    is_live = True # bool |  （可选）
    do_spam_check = True # bool |  （可选）
    skip_dup_check = True # bool |  （可选）

    try:
        api_response = api_instance.create_feed_post(tenant_id, create_feed_post_params, CreateFeedPostOptions(broadcast_id=broadcast_id, is_live=is_live, do_spam_check=do_spam_check, skip_dup_check=skip_dup_check))
        print("The response of DefaultApi->create_feed_post:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->create_feed_post: %s\n" % e)
[inline-code-end]