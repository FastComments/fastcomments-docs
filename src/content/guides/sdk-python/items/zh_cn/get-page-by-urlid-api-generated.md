## 参数

| 名称 | 类型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| urlId | string | query | 是 |  |

## 响应

返回：[`GetPageByURLIdAPIResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_page_by_urlid_api_response.py)

## 示例

[inline-code-attrs-start title = 'get_page_by_urlid 示例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_page_by_urlid_api_response import GetPageByURLIdAPIResponse
from client.rest import ApiException
from pprint import pprint

# 定义主机是可选的，默认为 https://fastcomments.com
# 有关所有支持的配置参数列表，请参阅 configuration.py。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# 客户端必须根据 API 服务器的安全策略配置身份验证和授权参数。
# 下面提供了每种身份验证方法的示例，请使用符合您用例的示例。

# 配置 API 密钥授权：api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 如有需要，请取消注释下面以为 API 密钥设置前缀（例如 Bearer）
# configuration.api_key_prefix['api_key'] = 'Bearer'

# 使用 API 客户端实例进入上下文
with client.ApiClient(configuration) as api_client:
    # 创建 API 类的实例
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | 

    try:
        api_response = api_instance.get_page_by_urlid(tenant_id, url_id)
        print("The response of DefaultApi->get_page_by_urlid:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_page_by_urlid: %s\n" % e)
[inline-code-end]