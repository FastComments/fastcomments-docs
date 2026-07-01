## 参数

| 名称 | 类型 | 位置 | 必需 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| tag | string | path | Yes |  |

## 响应

返回：[`APIEmptyResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_empty_response.py)

## 示例

[inline-code-attrs-start title = 'delete_hash_tag 示例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_empty_response import APIEmptyResponse
from client.models.delete_hash_tag_request_body import DeleteHashTagRequestBody
from client.rest import ApiException
from pprint import pprint

# 定义 host 是可选的，默认值为 https://fastcomments.com
# 请参阅 configuration.py 以获取所有受支持的配置参数列表。
# 客户端必须根据 API 服务器的安全策略配置身份验证和授权参数。
# 以下提供了每种身份验证方法的示例，请使用满足您认证使用情况的示例。

# 配置 API 密钥授权：api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 如有需要，取消以下注释以为 API 密钥设置前缀（例如 Bearer）  
# configuration.api_key_prefix['api_key'] = 'Bearer'

# 使用 API 客户端实例进入上下文
with client.ApiClient(configuration) as api_client:
    # 创建 API 类的实例
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    tag = 'tag_example' # str | 
    delete_hash_tag_request_body = client.DeleteHashTagRequestBody() # DeleteHashTagRequestBody |  (optional)

    try:
        api_response = api_instance.delete_hash_tag(tenant_id, tag, delete_hash_tag_request_body)
        print("DefaultApi->delete_hash_tag 的响应：\n")
        pprint(api_response)
    except Exception as e:
        print("调用 DefaultApi->delete_hash_tag 时出现异常：%s\n" % e)
[inline-code-end]