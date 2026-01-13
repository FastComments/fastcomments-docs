---
## 参数

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| skip | number | query | 否 |  |

## 响应

返回: [`GetTenantPackages200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_tenant_packages200_response.py)

## 示例

[inline-code-attrs-start title = 'get_tenant_packages 示例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_tenant_packages200_response import GetTenantPackages200Response
from client.rest import ApiException
from pprint import pprint

# 定义主机是可选的，默认值为 https://fastcomments.com
# 有关所有支持的配置参数列表，请参阅 configuration.py。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# 客户端必须根据 API 服务器的安全策略配置认证和授权参数。
# 下面提供了每种认证方法的示例，请使用适合您认证用例的示例。

# 配置 API 密钥授权：api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 如有需要，可取消下面行的注释以设置前缀（例如 Bearer）用于 API 密钥
# configuration.api_key_prefix['api_key'] = 'Bearer'

# 使用 API 客户端实例进入上下文
with client.ApiClient(configuration) as api_client:
    # 创建 API 类的一个实例
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    skip = 3.4 # float |  (可选)

    try:
        api_response = api_instance.get_tenant_packages(tenant_id, skip=skip)
        print("The response of DefaultApi->get_tenant_packages:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_tenant_packages: %s\n" % e)
[inline-code-end]

---