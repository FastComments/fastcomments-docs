## 参数

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes |  |
| id | string | query | Yes |  |

## 响应

返回: [`GetV2PageReactUsersResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_v2_page_react_users_response.py)

## 示例

[inline-code-attrs-start title = 'get_v2_page_react_users 示例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_v2_page_react_users_response import GetV2PageReactUsersResponse
from client.rest import ApiException
from pprint import pprint

# 定义主机是可选的，默认为 https://fastcomments.com
# 请参阅 configuration.py 以获取所有支持的配置参数的列表。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# 使用 API 客户端实例进入上下文
with client.ApiClient(configuration) as api_client:
    # 创建 API 类的实例
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | 
    id = 'id_example' # str | 

    try:
        api_response = api_instance.get_v2_page_react_users(tenant_id, url_id, id)
        print("The response of PublicApi->get_v2_page_react_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_v2_page_react_users: %s\n" % e)
[inline-code-end]