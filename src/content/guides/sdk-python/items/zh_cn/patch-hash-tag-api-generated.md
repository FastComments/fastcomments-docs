## 参数

| 名称 | 类型 | 位置 | 必需 | 描述 |
|------|------|----------|----------|-------------|
| tag | string | path | 是 |  |
| tenantId | string | query | 否 |  |

## 响应

返回: [`PatchHashTag200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/patch_hash_tag200_response.py)

## 示例

[inline-code-attrs-start title = 'patch_hash_tag 示例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.patch_hash_tag200_response import PatchHashTag200Response
from client.models.update_hash_tag_body import UpdateHashTagBody
from client.rest import ApiException
from pprint import pprint

# 定义主机是可选的，默认值为 https://fastcomments.com
# 请参阅 configuration.py 以获取所有受支持的配置参数列表。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# 客户端必须配置身份验证和授权参数
# 以符合 API 服务器的安全策略。
# 每种身份验证方法的示例在下面提供，请使用满足您身份验证用例的示例。

# 配置 API 密钥授权：api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 如有需要，取消注释下面的行以设置 API 密钥的前缀（例如 Bearer）
# configuration.api_key_prefix['api_key'] = 'Bearer'

# 使用 API 客户端实例进入上下文
with client.ApiClient(configuration) as api_client:
    # 创建 API 类的实例
    api_instance = client.DefaultApi(api_client)
    tag = 'tag_example' # str | 
    tenant_id = 'tenant_id_example' # str |  (可选)
    update_hash_tag_body = client.UpdateHashTagBody() # UpdateHashTagBody |  (可选)

    try:
        api_response = api_instance.patch_hash_tag(tag, tenant_id=tenant_id, update_hash_tag_body=update_hash_tag_body)
        print("The response of DefaultApi->patch_hash_tag:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->patch_hash_tag: %s\n" % e)
[inline-code-end]