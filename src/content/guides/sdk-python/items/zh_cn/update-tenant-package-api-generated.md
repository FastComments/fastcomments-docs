## 参数

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| id | string | path | 是 |  |

## 响应

返回: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_empty_response.py)

## 示例

[inline-code-attrs-start title = 'update_tenant_package 示例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_empty_response import APIEmptyResponse
from client.models.update_tenant_package_body import UpdateTenantPackageBody
from client.rest import ApiException
from pprint import pprint

# 定义主机是可选的，默认值为 https://fastcomments.com
# 请参阅 configuration.py 以获取所有支持的配置参数列表。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# 客户端必须根据 API 服务器的安全策略配置身份验证和授权参数。
# 下面提供了每种认证方法的示例，请使用满足您认证用例的示例。
# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 如有需要，取消下面的注释以为 API 密钥设置前缀（例如 Bearer）
# configuration.api_key_prefix['api_key'] = 'Bearer'

# 使用 API 客户端实例进入上下文
with client.ApiClient(configuration) as api_client:
    # 创建 API 类的实例
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    update_tenant_package_body = client.UpdateTenantPackageBody() # UpdateTenantPackageBody | 

    try:
        api_response = api_instance.update_tenant_package(tenant_id, id, update_tenant_package_body)
        print("The response of DefaultApi->update_tenant_package:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->update_tenant_package: %s\n" % e)
[inline-code-end]