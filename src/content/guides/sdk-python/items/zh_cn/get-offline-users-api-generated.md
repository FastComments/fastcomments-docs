Past commenters on the page who are NOT currently online. Sorted by displayName.  
页面上过去的评论者（当前不在线）。按 displayName 排序。

Use this after exhausting /users/online to render a "Members" section.  
在耗尽 /users/online 后使用此接口来渲染 “Members”（成员） 部分。

Cursor pagination on commenterName: server walks the partial {tenantId, urlId, commenterName} index from afterName forward via $gt, no $skip cost.  
基于 commenterName 的游标分页：服务器从 afterName 起向前遍历部分 {tenantId, urlId, commenterName} 索引，使用 $gt，无需 $skip 开销。

## Parameters

| 名称 | 类型 | 位置 | 必需 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | 页面 URL 标识符（已在服务器端清理）。 |
| afterName | string | query | No | 游标：传入上一次响应中的 nextAfterName。 |
| afterUserId | string | query | No | 游标平局破拆：传入上一次响应中的 nextAfterUserId。若 afterName 已设置，则此项为必需，以防名称相同导致条目被丢弃。 |

## Response

返回: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/page_users_offline_response.py)

## Example

[inline-code-attrs-start title = 'get_offline_users 示例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import GetOfflineUsersOptions
from client.models.page_users_offline_response import PageUsersOfflineResponse
from client.rest import ApiException
from pprint import pprint

# 定义主机是可选的，默认值为 https://fastcomments.com
# 请参阅 configuration.py 了解所有受支持的配置参数列表。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# 使用 API 客户端实例进入上下文
with client.ApiClient(configuration) as api_client:
    # 创建 API 类的实例
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | 页面 URL 标识符（已在服务器端清理）。
    after_name = 'after_name_example' # str | 游标：传入上一次响应中的 nextAfterName。（可选）
    after_user_id = 'after_user_id_example' # str | 游标平局破拆：传入上一次响应中的 nextAfterUserId。若 afterName 已设置，则此项为必需，以防名称相同导致条目被丢弃。（可选）

    try:
        api_response = api_instance.get_offline_users(tenant_id, url_id, GetOfflineUsersOptions(after_name=after_name, after_user_id=after_user_id))
        print("The response of PublicApi->get_offline_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_offline_users: %s\n" % e)
[inline-code-end]