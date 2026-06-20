## 参数

| 名称 | 类型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 是 |  |
| urlId | string | query | 是 |  |

## 响应

返回：[`CreateV1PageReact`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/create_v1_page_react.py)

## 示例

[inline-code-attrs-start title = 'delete_v1_page_react 示例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.create_v1_page_react import CreateV1PageReact
from client.rest import ApiException
from pprint import pprint

# 定义主机是可选的，默认为 https://fastcomments.com
# 有关所有受支持配置参数的列表，请参阅 configuration.py。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# 使用 API 客户端实例进入上下文
with client.ApiClient(configuration) as api_client:
    # 创建 API 类的实例
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | 

    try:
        api_response = api_instance.delete_v1_page_react(tenant_id, url_id)
        print("The response of PublicApi->delete_v1_page_react:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->delete_v1_page_react: %s\n" % e)
[inline-code-end]

---