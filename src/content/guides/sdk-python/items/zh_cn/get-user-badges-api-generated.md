## 参数

| 名称 | 类型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| userId | string | query | 否 |  |
| badgeId | string | query | 否 |  |
| type | number | query | 否 |  |
| displayedOnComments | boolean | query | 否 |  |
| limit | number | query | 否 |  |
| skip | number | query | 否 |  |

## 响应

返回：[`APIGetUserBadgesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_get_user_badges_response.py)

## 示例

[inline-code-attrs-start title = '获取用户徽章 示例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.default_api import GetUserBadgesOptions
from client.models.api_get_user_badges_response import APIGetUserBadgesResponse
from client.rest import ApiException
from pprint import pprint

# 定义 host 是可选的，默认值为 https://fastcomments.com
# 查看 configuration.py 以获取所有受支持的配置参数列表。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# 客户端必须根据 API 服务器的安全策略配置身份验证和授权参数。
# 以下提供了每种认证方法的示例，请使用符合您认证需求的示例。

# 配置 API 密钥授权：api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 如有需要，取消注释下面的代码以设置 API 密钥前缀（例如 Bearer）
# configuration.api_key_prefix['api_key'] = 'Bearer'

# 使用 API 客户端实例进入上下文
with client.ApiClient(configuration) as api_client:
    # 创建 API 类的实例
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    user_id = 'user_id_example' # str |  （可选）
    badge_id = 'badge_id_example' # str |  （可选）
    type = 3.4 # float |  （可选）
    displayed_on_comments = True # bool |  （可选）
    limit = 3.4 # float |  （可选）
    skip = 3.4 # float |  （可选）

    try:
        api_response = api_instance.get_user_badges(tenant_id, GetUserBadgesOptions(user_id=user_id, badge_id=badge_id, type=type, displayed_on_comments=displayed_on_comments, limit=limit, skip=skip))
        print("DefaultApi->get_user_badges 的响应结果：\n")
        pprint(api_response)
    except Exception as e:
        print("调用 DefaultApi->get_user_badges 时出现异常：%s\n" % e)
[inline-code-end]