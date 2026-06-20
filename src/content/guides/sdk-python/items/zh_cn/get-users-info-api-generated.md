---
针对租户的批量用户信息。给定 userIds，返回来自 User / SSOUser 的显示信息。
由评论小部件用于丰富刚通过 presence 事件出现的用户。
无页面上下文：隐私统一强制执行（私人资料将被屏蔽）。

## 参数

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| ids | string | query | Yes | 逗号分隔的 userIds。 |

## 响应

返回: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/page_users_info_response.py)

## 示例

[inline-code-attrs-start title = 'get_users_info 示例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.page_users_info_response import PageUsersInfoResponse
from client.rest import ApiException
from pprint import pprint

# 定义主机是可选的，默认值为 https://fastcomments.com
# 有关所有支持的配置参数列表，请参阅 configuration.py。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# 使用 API 客户端实例进入上下文
with client.ApiClient(configuration) as api_client:
    # 创建 API 类的实例
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    ids = 'ids_example' # str | 逗号分隔的 userIds。

    try:
        api_response = api_instance.get_users_info(tenant_id, ids)
        print("The response of PublicApi->get_users_info:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_users_info: %s\n" % e)
[inline-code-end]

---