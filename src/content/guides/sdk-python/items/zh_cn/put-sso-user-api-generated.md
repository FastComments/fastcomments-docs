## 参数

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| id | string | path | 是 |  |
| updateComments | boolean | query | 否 |  |

## 响应

返回: [`PutSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/put_sso_user_api_response.py)

## 示例

[inline-code-attrs-start title = 'put_sso_user 示例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.put_sso_user_api_response import PutSSOUserAPIResponse
from client.models.update_apisso_user_data import UpdateAPISSOUserData
from client.rest import ApiException
from pprint import pprint

# 定义主机是可选的，默认值为 https://fastcomments.com
# 有关所有支持的配置参数列表，请参阅 configuration.py。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# 客户端必须根据 API 服务器的安全策略配置认证和授权参数。
# 下面为每种认证方法提供示例，请使用满足您认证用例的示例。

# 配置 API 密钥授权：api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 如有需要，请取消注释下面以为 API 密钥设置前缀（例如 Bearer）
# configuration.api_key_prefix['api_key'] = 'Bearer'

# 使用 API 客户端的实例进入上下文
with client.ApiClient(configuration) as api_client:
    # 创建 API 类的一个实例
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    update_apisso_user_data = client.UpdateAPISSOUserData() # UpdateAPISSOUserData | 
    update_comments = True # bool |  (可选)

    try:
        api_response = api_instance.put_sso_user(tenant_id, id, update_apisso_user_data, update_comments=update_comments)
        print("The response of DefaultApi->put_sso_user:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->put_sso_user: %s\n" % e)
[inline-code-end]