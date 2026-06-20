页面当前在线的查看者：当前其 websocket 会话已订阅该页面的人。
返回 anonCount + totalCount（房间范围的订阅者，包括我们不逐一列举的匿名查看者）。

## 参数

| 名称 | 类型 | 位置 | 必需 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 是 |  |
| urlId | string | query | 是 | 页面 URL 标识符（由服务器端清理）。 |
| afterName | string | query | 否 | 游标：传入前一个响应中的 nextAfterName。 |
| afterUserId | string | query | 否 | 游标决胜项：传入前一个响应中的 nextAfterUserId。 当设置了 afterName 时为必需，以防名称相同导致条目丢失。 |

## 响应

返回: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/page_users_online_response.py)

## 示例

[inline-code-attrs-start title = 'get_online_users 示例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.page_users_online_response import PageUsersOnlineResponse
from client.rest import ApiException
from pprint import pprint

# 定义 host 为可选，默认值为 https://fastcomments.com
# 有关所有支持的配置参数列表，请参见 configuration.py。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# 使用 ApiClient 实例进入上下文
with client.ApiClient(configuration) as api_client:
    # 创建 API 类的一个实例
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | 页面 URL 标识符（由服务器端清理）。
    after_name = 'after_name_example' # str | 游标：传入前一个响应中的 nextAfterName。（可选）
    after_user_id = 'after_user_id_example' # str | 游标决胜项：传入前一个响应中的 nextAfterUserId。当设置了 afterName 时为必需，以防名称相同导致条目丢失。（可选）

    try:
        api_response = api_instance.get_online_users(tenant_id, url_id, after_name=after_name, after_user_id=after_user_id)
        print("The response of PublicApi->get_online_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_online_users: %s\n" % e)
[inline-code-end]