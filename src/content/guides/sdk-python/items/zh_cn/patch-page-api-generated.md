## 参数

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| id | string | path | 是 |  |

## 响应

返回: [`PatchPageAPIResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/patch_page_api_response.py)

## 示例

[inline-code-attrs-start title = 'patch_page 示例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.patch_page_api_response import PatchPageAPIResponse
from client.models.update_api_page_data import UpdateAPIPageData
from client.rest import ApiException
from pprint import pprint

# 定义主机是可选的，默认为 https://fastcomments.com
# 有关所有支持的配置参数列表，请参阅 configuration.py。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# 客户端必须根据 API 服务器的安全策略配置身份验证和授权参数。
# 下面提供了每种身份验证方法的示例，请使用满足您身份验证用例的示例。

# 配置 API 密钥授权：api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 如有需要，取消注释下面以为 API 密钥设置前缀（例如 Bearer）
# configuration.api_key_prefix['api_key'] = 'Bearer'

# 使用 API 客户端实例进入上下文
with client.ApiClient(configuration) as api_client:
    # 创建 API 类的实例
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    update_api_page_data = client.UpdateAPIPageData() # UpdateAPIPageData | 

    try:
        api_response = api_instance.patch_page(tenant_id, id, update_api_page_data)
        print("The response of DefaultApi->patch_page:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->patch_page: %s\n" % e)
[inline-code-end]

---