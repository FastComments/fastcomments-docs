## 参数

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| yearNumber | number | query | 否 |  |
| monthNumber | number | query | 否 |  |
| dayNumber | number | query | 否 |  |
| skip | number | query | 否 |  |

## 响应

返回：[`GetTenantDailyUsages200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_tenant_daily_usages200_response.py)

## 示例

[inline-code-attrs-start title = 'get_tenant_daily_usages 示例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_tenant_daily_usages200_response import GetTenantDailyUsages200Response
from client.rest import ApiException
from pprint import pprint

# 定义主机是可选的，默认值为 https://fastcomments.com
# 有关所有支持的配置参数列表，请参阅 configuration.py。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# 客户端必须配置认证和授权参数
# 以符合 API 服务器的安全策略。
# 每种认证方法的示例在下面提供，
# 使用满足您认证用例的示例。

# 配置 API 密钥授权：api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 如有需要，取消注释下面以设置 API 密钥的前缀（例如 Bearer）
# configuration.api_key_prefix['api_key'] = 'Bearer'

# 使用 API 客户端实例进入上下文
with client.ApiClient(configuration) as api_client:
    # 创建 API 类的实例
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    year_number = 3.4 # float |  (可选)
    month_number = 3.4 # float |  (可选)
    day_number = 3.4 # float |  (可选)
    skip = 3.4 # float |  (可选)

    try:
        api_response = api_instance.get_tenant_daily_usages(tenant_id, year_number=year_number, month_number=month_number, day_number=day_number, skip=skip)
        print("The response of DefaultApi->get_tenant_daily_usages:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_tenant_daily_usages: %s\n" % e)
[inline-code-end]

---