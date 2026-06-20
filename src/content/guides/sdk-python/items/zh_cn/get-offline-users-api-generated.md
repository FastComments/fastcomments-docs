页面上已发表评论但当前不在线的用户。按 displayName 排序。
在用尽 /users/online 后使用此接口以呈现 "Members" 部分。
基于 commenterName 的游标分页：服务器沿着部分 {tenantId, urlId, commenterName} 索引从 afterName 向前通过 $gt 遍历，无 $skip 成本。

## 参数

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | 页面 URL 标识符（服务器端已清理）。 |
| afterName | string | query | No | 游标：传入上一响应中的 nextAfterName。 |
| afterUserId | string | query | No | 游标平局判定器：传入上一响应中的 nextAfterUserId。当设置了 afterName 时需要传入，以避免同名导致条目丢失。 |

## 响应

返回: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/page_users_offline_response.py)

## 示例

[inline-code-attrs-start title = 'get_offline_users 示例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.page_users_offline_response import PageUsersOfflineResponse
from client.rest import ApiException
from pprint import pprint

# 定义主机是可选的，默认值为 https://fastcomments.com
# 有关所有支持的配置参数的列表，请参见 configuration.py。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# 使用 API 客户端实例进入上下文
with client.ApiClient(configuration) as api_client:
    # 创建 API 类的一个实例
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | 页面 URL 标识符（服务器端已清理）。
    after_name = 'after_name_example' # str | 游标：传入上一响应中的 nextAfterName。（可选）
    after_user_id = 'after_user_id_example' # str | 游标平局判定器：传入上一响应中的 nextAfterUserId。当设置了 afterName 时需要传入，以避免同名导致条目丢失。（可选）

    try:
        api_response = api_instance.get_offline_users(tenant_id, url_id, after_name=after_name, after_user_id=after_user_id)
        print("The response of PublicApi->get_offline_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_offline_users: %s\n" % e)
[inline-code-end]