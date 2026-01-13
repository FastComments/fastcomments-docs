## 参数

| 名称 | 类型 | 位置 | 必需 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 否 |  |

## 响应

返回: [`AddHashTag200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/add_hash_tag200_response.py)

## 示例

[inline-code-attrs-start title = 'add_hash_tag 示例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.add_hash_tag200_response import AddHashTag200Response
from client.models.create_hash_tag_body import CreateHashTagBody
from client.rest import ApiException
from pprint import pprint

# 指定主机是可选的，默认使用 https://fastcomments.com
# 请参见 configuration.py 以获取所有受支持配置参数的列表。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# 客户端必须根据 API 服务器的安全策略配置身份验证和授权参数。
# 下方提供了每种身份验证方法的示例，请使用满足您认证用例的示例。
# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 如有需要，解除下面的注释以为 API 密钥设置前缀（例如 Bearer）
# configuration.api_key_prefix['api_key'] = 'Bearer'

# 使用 API 客户端实例进入上下文
with client.ApiClient(configuration) as api_client:
    # 创建 API 类的一个实例
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str |  (可选)
    create_hash_tag_body = client.CreateHashTagBody() # CreateHashTagBody |  (可选)

    try:
        api_response = api_instance.add_hash_tag(tenant_id=tenant_id, create_hash_tag_body=create_hash_tag_body)
        print("The response of DefaultApi->add_hash_tag:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->add_hash_tag: %s\n" % e)
[inline-code-end]