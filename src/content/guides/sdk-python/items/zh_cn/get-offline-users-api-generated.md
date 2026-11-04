当前不在线的页面历史评论者。按 displayName 排序。
在用完 /users/online 之后使用此接口来渲染“成员”部分。
基于 commenterName 的游标分页：服务器通过 $gt 从 afterName 开始向前遍历部分索引 {tenantId, urlId, commenterName}，没有 $skip 开销。

## 参数

| 名称 | 类型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 是 |  |
| urlId | string | query | 是 | 页面 URL 标识符（在服务端进行清理）。 |
| afterName | string | query | 否 | 游标：传入上一次响应中的 nextAfterName。 |
| afterUserId | string | query | 否 | 游标决胜键：传入上一次响应中的 nextAfterUserId。当设置了 afterName 时必填，以避免同名条目被丢弃。 |

## 响应

返回：[`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/page_users_offline_response.py)

## 示例

[inline-code-attrs-start title = 'get_offline_users 示例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import GetOfflineUsersOptions
from client.models.page_users_offline_response import PageUsersOfflineResponse
from client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to https://fastcomments.com
# See configuration.py for a list of all supported configuration parameters.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Enter a context with an instance of the API client
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | 页面 URL 标识符（在服务端进行清理）。
    after_name = 'after_name_example' # str | 游标：传入上一次响应中的 nextAfterName。 (optional)
    after_user_id = 'after_user_id_example' # str | 游标决胜键：传入上一次响应中的 nextAfterUserId。当设置了 afterName 时必填，以避免同名条目被丢弃。 (optional)

    try:
        api_response = api_instance.get_offline_users(tenant_id, url_id, GetOfflineUsersOptions(after_name=after_name, after_user_id=after_user_id))
        print("The response of PublicApi->get_offline_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_offline_users: %s\n" % e)
[inline-code-end]