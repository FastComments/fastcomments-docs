## 参数

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | 查询 | 是 |  |

## 响应

返回: [`GetEmailTemplateDefinitionsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_email_template_definitions_response.py)

## 示例

[inline-code-attrs-start title = 'get_email_template_definitions 示例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_email_template_definitions_response import GetEmailTemplateDefinitionsResponse
from client.rest import ApiException
from pprint import pprint

# 定义主机是可选的，默认为 https://fastcomments.com
# 有关所有支持的配置参数列表，请参阅 configuration.py。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# 客户端必须按照 API 服务器的安全策略配置身份验证和授权参数。
# 下方提供了每种身份验证方法的示例，请使用满足您用例的示例。
# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 如有需要，取消注释下面行以为 API 密钥设置前缀（例如 Bearer）
# configuration.api_key_prefix['api_key'] = 'Bearer'

# 在 API 客户端实例的上下文中
with client.ApiClient(configuration) as api_client:
    # 创建 API 类的实例
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 

    try:
        api_response = api_instance.get_email_template_definitions(tenant_id)
        print("The response of DefaultApi->get_email_template_definitions:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_email_template_definitions: %s\n" % e)
[inline-code-end]