## 参数

| 名称 | 类型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| userId | string | query | No |  |
| limit | number | query | No |  |
| skip | number | query | No |  |

## 响应

返回：[`APIGetUserBadgeProgressListResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_get_user_badge_progress_list_response.py)

## 示例

[inline-code-attrs-start title = 'get_user_badge_progress_list 示例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.default_api import GetUserBadgeProgressListOptions
from client.models.api_get_user_badge_progress_list_response import APIGetUserBadgeProgressListResponse
from client.rest import ApiException
from pprint import pprint

# 定义主机是可选的，默认值为 https://fastcomments.com
# 请查看 configuration.py 获取所有受支持的配置参数列表。
# 客户端必须根据 API 服务器的安全策略配置身份验证和授权参数。
# 以下提供了每种身份验证方法的示例，请使用适合您身份验证使用场景的示例。

# 配置 API key 授权：api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 如有需要，取消注释下面代码为 API key 设置前缀（例如 Bearer）
# configuration.api_key_prefix['api_key'] = 'Bearer'

# 使用 API 客户端实例进入上下文
with client.ApiClient(configuration) as api_client:
    # 创建 API 类的实例
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str |
    user_id = 'user_id_example' # str |  (可选)
    limit = 3.4 # float |  (可选)
    skip = 3.4 # float |  (可选)

    try:
        api_response = api_instance.get_user_badge_progress_list(tenant_id, GetUserBadgeProgressListOptions(user_id=user_id, limit=limit, skip=skip))
        print("The response of DefaultApi->get_user_badge_progress_list:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_user_badge_progress_list: %s\n" % e)
[inline-code-end]