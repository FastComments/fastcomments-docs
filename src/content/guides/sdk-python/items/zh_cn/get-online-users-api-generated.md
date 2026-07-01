目前在线的页面查看者：当前其 websocket 会话已订阅该页面的用户。  
返回 anonCount + totalCount（整个房间的订阅者数量，包括我们未枚举的匿名查看者）。

## 参数

| 名称 | 类型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 是 |  |
| urlId | string | query | 是 | 页面 URL 标识符（服务器端已清理）。 |
| afterName | string | query | 否 | 游标：传入上一响应中的 nextAfterName。 |
| afterUserId | string | query | 否 | 游标平衡键：传入上一响应中的 nextAfterUserId。设置 afterName 时必填，以防名称相同导致条目被遗漏。 |

## 响应

返回: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/page_users_online_response.py)

## 示例

[inline-code-attrs-start title = 'get_online_users 示例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import GetOnlineUsersOptions
from client.models.page_users_online_response import PageUsersOnlineResponse
from client.rest import ApiException
from pprint import pprint

# 定义主机是可选的，默认是 https://fastcomments.com
# 请查看 configuration.py 以获取所有受支持的配置参数列表。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# 使用 API 客户端实例进入上下文
with client.ApiClient(configuration) as api_client:
    # 创建 API 类的实例
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | 页面 URL 标识符（服务器端已清理）。
    after_name = 'after_name_example' # str | 游标：传入上一响应中的 nextAfterName。（可选）
    after_user_id = 'after_user_id_example' # str | 游标平衡键：传入上一响应中的 nextAfterUserId。设置 afterName 时必填，以防名称相同导致条目被遗漏。（可选）

    try:
        api_response = api_instance.get_online_users(tenant_id, url_id, GetOnlineUsersOptions(after_name=after_name, after_user_id=after_user_id))
        print("The response of PublicApi->get_online_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_online_users: %s\n" % e)
[inline-code-end]